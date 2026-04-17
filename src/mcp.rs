use std::collections::BTreeSet;
use std::path::PathBuf;

use rmcp::{
    ErrorData as McpError, Json, ServerHandler, ServiceExt,
    handler::server::wrapper::Parameters,
    model::{Implementation, ServerCapabilities, ServerInfo},
    service::{RequestContext, RoleServer},
    tool, tool_handler, tool_router,
    transport::stdio,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::auth::load_credentials;
use crate::cli::{PullRequestState, RepoRole};
use crate::client::BitbucketClient;
use crate::error::BbcliError;
use crate::git;
use crate::model::{PullRequestJsonRow, RepoJsonRow};

const BITBUCKET_MACHINE: &str = "api.bitbucket.org";

#[derive(Clone)]
pub struct BbMcpServer {
    login: String,
    token: String,
}

impl BbMcpServer {
    fn new(login: String, token: String) -> Self {
        Self { login, token }
    }

    fn client(&self) -> BitbucketClient {
        BitbucketClient::from_credentials(self.login.clone(), self.token.clone())
    }
}

#[tool_handler(router = Self::tool_router())]
impl ServerHandler for BbMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::from_build_env())
            .with_instructions("Bitbucket MCP server. Use bitbucket.list_repositories to discover repositories. Use bitbucket.list_pull_requests with explicit workspace_slug and repo_slug when known; otherwise it will resolve the repository from MCP roots.")
    }
}

#[tool_router(router = tool_router)]
impl BbMcpServer {
    #[tool(
        name = "bitbucket.list_repositories",
        description = "List repositories the authenticated user can access.",
        annotations(read_only_hint = true)
    )]
    async fn list_repositories(
        &self,
        Parameters(params): Parameters<ListRepositoriesParams>,
    ) -> Result<Json<ListRepositoriesResult>, McpError> {
        let repositories = self
            .client()
            .list_accessible_repositories(
                params.workspace_slug.as_deref(),
                params.role.map(|role| role.as_api_value()),
            )
            .await
            .map_err(|err| map_runtime_error("listing repositories", err))?;

        Ok(Json(ListRepositoriesResult {
            repositories: repositories.iter().map(RepoJsonRow::from).collect(),
        }))
    }

    #[tool(
        name = "bitbucket.list_pull_requests",
        description = "List pull requests for a Bitbucket repository. When workspace_slug and repo_slug are omitted, the server resolves the repository from the client's MCP roots.",
        annotations(read_only_hint = true)
    )]
    async fn list_pull_requests(
        &self,
        Parameters(params): Parameters<ListPullRequestsParams>,
        context: RequestContext<RoleServer>,
    ) -> Result<Json<ListPullRequestsResult>, McpError> {
        let (workspace_slug, repo_slug) = match (
            params.workspace_slug.as_deref(),
            params.repo_slug.as_deref(),
        ) {
            (Some(workspace_slug), Some(repo_slug)) => {
                (workspace_slug.to_owned(), repo_slug.to_owned())
            }
            (None, None) => resolve_repo_from_context(context).await?,
            _ => {
                return Err(McpError::invalid_params(
                    "workspace_slug and repo_slug must be provided together",
                    None,
                ));
            }
        };

        let pull_requests = self
            .client()
            .list_pull_requests_in_repository(
                workspace_slug.as_str(),
                repo_slug.as_str(),
                params.state.map(|state| state.as_api_value()),
            )
            .await
            .map_err(|err| map_runtime_error("listing pull requests", err))?;

        Ok(Json(ListPullRequestsResult {
            pull_requests: pull_requests.iter().map(PullRequestJsonRow::from).collect(),
        }))
    }
}

#[derive(Debug, Deserialize, JsonSchema)]
struct ListRepositoriesParams {
    workspace_slug: Option<String>,
    role: Option<RepoRole>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct ListPullRequestsParams {
    workspace_slug: Option<String>,
    repo_slug: Option<String>,
    state: Option<PullRequestState>,
}

#[derive(Debug, Serialize, JsonSchema)]
struct ListRepositoriesResult {
    repositories: Vec<RepoJsonRow>,
}

#[derive(Debug, Serialize, JsonSchema)]
struct ListPullRequestsResult {
    pull_requests: Vec<PullRequestJsonRow>,
}

pub async fn run_server() -> Result<(), BbcliError> {
    let credentials = load_credentials(BITBUCKET_MACHINE)?;
    let server = BbMcpServer::new(credentials.login, credentials.token);
    let service = server.serve(stdio()).await.map_err(|err| BbcliError::Mcp {
        context: "starting MCP server",
        message: err.to_string(),
    })?;

    service.waiting().await.map_err(|err| BbcliError::Mcp {
        context: "running MCP server",
        message: err.to_string(),
    })?;

    Ok(())
}

async fn resolve_repo_from_context(
    context: RequestContext<RoleServer>,
) -> Result<(String, String), McpError> {
    let roots = context.peer.list_roots().await.map_err(|err| {
        McpError::invalid_params(
            format!(
                "unable to resolve a repository from MCP roots: client did not provide roots ({err})"
            ),
            None,
        )
    })?;

    infer_repo_from_roots(roots.roots.as_slice()).map_err(|err| McpError::invalid_params(err, None))
}

fn infer_repo_from_roots(roots: &[rmcp::model::Root]) -> Result<(String, String), String> {
    if roots.is_empty() {
        return Err(
            "unable to resolve a repository from MCP roots: no roots were provided".to_owned(),
        );
    }

    let mut candidates = BTreeSet::new();

    for root in roots {
        let Some(path) = file_root_to_path(root) else {
            continue;
        };

        match git::detect_bitbucket_repo_slug_from_path(path.as_path()) {
            Ok(repo) => {
                candidates.insert(repo);
            }
            Err(BbcliError::GitRepoNotFound) | Err(BbcliError::BitbucketRemoteNotFound) => {}
            Err(err) => {
                return Err(format!(
                    "unable to resolve a repository from root `{}`: {err}",
                    root.uri
                ));
            }
        }
    }

    match candidates.len() {
        0 => Err(
            "unable to resolve a repository from MCP roots: no Bitbucket repository was found"
                .to_owned(),
        ),
        1 => Ok(candidates.into_iter().next().expect("single candidate")),
        _ => Err(format!(
            "unable to resolve a repository from MCP roots: multiple Bitbucket repositories matched ({})",
            candidates
                .into_iter()
                .map(|(workspace, repo)| format!("{workspace}/{repo}"))
                .collect::<Vec<_>>()
                .join(", ")
        )),
    }
}

fn file_root_to_path(root: &rmcp::model::Root) -> Option<PathBuf> {
    let url = Url::parse(root.uri.as_str()).ok()?;
    if url.scheme() != "file" {
        return None;
    }

    url.to_file_path().ok()
}

fn map_runtime_error(context: &'static str, err: BbcliError) -> McpError {
    McpError::internal_error(format!("error {context}: {err}"), None)
}

#[cfg(test)]
mod tests {
    use rmcp::model::Root;

    use super::{file_root_to_path, infer_repo_from_roots};

    fn file_root(path: &str) -> Root {
        Root::new(format!("file://{path}"))
    }

    #[test]
    fn file_root_to_path_ignores_non_file_uris() {
        let root = Root::new("https://bitbucket.org/acme/api");
        assert!(file_root_to_path(&root).is_none());
    }

    #[test]
    fn infer_repo_from_roots_rejects_empty_roots() {
        let err = infer_repo_from_roots(&[]).expect_err("empty roots should fail");
        assert!(err.contains("no roots were provided"));
    }

    #[test]
    fn infer_repo_from_roots_rejects_when_no_bitbucket_repo_is_found() {
        let tmp = std::env::temp_dir().join("bb-mcp-no-repo");
        std::fs::create_dir_all(&tmp).expect("temp dir should exist");
        let err = infer_repo_from_roots(&[file_root(tmp.to_str().expect("utf8 path"))])
            .expect_err("non-repo roots should fail");
        assert!(err.contains("no Bitbucket repository was found"));
    }
}
