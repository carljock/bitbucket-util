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
use crate::model::{
    PipelineDetailedJsonRow, PipelineJsonRow, PipelineStepJsonRow, PullRequestCommentJsonRow,
    PullRequestDetailedJsonRow, PullRequestJsonRow, RepoJsonRow,
};

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
            .with_instructions("Bitbucket MCP server providing repository, pull request, and pipeline management. Use bitbucket.list_repositories to discover repositories. For pull requests and pipelines, provide explicit workspace_slug and repo_slug, or the server will resolve the repository from MCP roots. Pipeline tools include: list, get details, trigger (with variables and custom selectors), stop, list steps, get step details, and fetch logs.")
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
        let (workspace_slug, repo_slug) = resolve_workspace_repo(params.workspace_slug, params.repo_slug, context).await?;

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

    #[tool(
        name = "bitbucket.get_pull_request",
        description = "Get detailed information about a single pull request.",
        annotations(read_only_hint = true)
    )]
    async fn get_pull_request(
        &self,
        Parameters(params): Parameters<GetPullRequestParams>,
        context: RequestContext<RoleServer>,
    ) -> Result<Json<PullRequestDetailedJsonRow>, McpError> {
        let (workspace_slug, repo_slug) = resolve_workspace_repo(params.workspace_slug, params.repo_slug, context).await?;

        let pr = self
            .client()
            .get_pull_request(&workspace_slug, &repo_slug, params.pull_request_id)
            .await
            .map_err(|err| map_runtime_error("getting pull request", err))?;

        Ok(Json(PullRequestDetailedJsonRow::from(&pr)))
    }

    #[tool(
        name = "bitbucket.get_pull_request_diff",
        description = "Get the raw diff of a pull request.",
        annotations(read_only_hint = true)
    )]
    async fn get_pull_request_diff(
        &self,
        Parameters(params): Parameters<GetPullRequestParams>,
        context: RequestContext<RoleServer>,
    ) -> Result<String, McpError> {
        let (workspace_slug, repo_slug) = resolve_workspace_repo(params.workspace_slug, params.repo_slug, context).await?;

        let diff = self
            .client()
            .get_pull_request_diff(&workspace_slug, &repo_slug, params.pull_request_id)
            .await
            .map_err(|err| map_runtime_error("getting pull request diff", err))?;

        Ok(diff)
    }

    #[tool(
        name = "bitbucket.list_pull_request_comments",
        description = "List all comments on a pull request.",
        annotations(read_only_hint = true)
    )]
    async fn list_pull_request_comments(
        &self,
        Parameters(params): Parameters<GetPullRequestParams>,
        context: RequestContext<RoleServer>,
    ) -> Result<Json<ListPullRequestCommentsResult>, McpError> {
        let (workspace_slug, repo_slug) = resolve_workspace_repo(params.workspace_slug, params.repo_slug, context).await?;

        let comments = self
            .client()
            .list_pull_request_comments(&workspace_slug, &repo_slug, params.pull_request_id)
            .await
            .map_err(|err| map_runtime_error("listing pull request comments", err))?;

        Ok(Json(ListPullRequestCommentsResult {
            comments: comments.iter().map(PullRequestCommentJsonRow::from).collect(),
        }))
    }

    #[tool(
        name = "bitbucket.create_pull_request_comment",
        description = "Create a comment on a pull request. Can be a global comment or an inline comment on a specific file and line."
    )]
    async fn create_pull_request_comment(
        &self,
        Parameters(params): Parameters<CreatePullRequestCommentParams>,
        context: RequestContext<RoleServer>,
    ) -> Result<Json<PullRequestCommentJsonRow>, McpError> {
        let (workspace_slug, repo_slug) = resolve_workspace_repo(params.workspace_slug, params.repo_slug, context).await?;

        let comment = self
            .client()
            .create_pull_request_comment(
                &workspace_slug,
                &repo_slug,
                params.pull_request_id,
                &params.content,
                params.inline,
            )
            .await
            .map_err(|err| map_runtime_error("creating pull request comment", err))?;

        Ok(Json(PullRequestCommentJsonRow::from(&comment)))
    }

    #[tool(
        name = "bitbucket.approve_pull_request",
        description = "Approve a pull request."
    )]
    async fn approve_pull_request(
        &self,
        Parameters(params): Parameters<GetPullRequestParams>,
        context: RequestContext<RoleServer>,
    ) -> Result<String, McpError> {
        let (workspace_slug, repo_slug) = resolve_workspace_repo(params.workspace_slug, params.repo_slug, context).await?;

        self.client()
            .approve_pull_request(&workspace_slug, &repo_slug, params.pull_request_id)
            .await
            .map_err(|err| map_runtime_error("approving pull request", err))?;

        Ok(format!("Pull request #{} approved.", params.pull_request_id))
    }

    #[tool(
        name = "bitbucket.unapprove_pull_request",
        description = "Unapprove a pull request."
    )]
    async fn unapprove_pull_request(
        &self,
        Parameters(params): Parameters<GetPullRequestParams>,
        context: RequestContext<RoleServer>,
    ) -> Result<String, McpError> {
        let (workspace_slug, repo_slug) = resolve_workspace_repo(params.workspace_slug, params.repo_slug, context).await?;

        self.client()
            .unapprove_pull_request(&workspace_slug, &repo_slug, params.pull_request_id)
            .await
            .map_err(|err| map_runtime_error("unapproving pull request", err))?;

        Ok(format!("Pull request #{} unapproved.", params.pull_request_id))
    }

    #[tool(
        name = "bitbucket.decline_pull_request",
        description = "Decline a pull request."
    )]
    async fn decline_pull_request(
        &self,
        Parameters(params): Parameters<GetPullRequestParams>,
        context: RequestContext<RoleServer>,
    ) -> Result<String, McpError> {
        let (workspace_slug, repo_slug) = resolve_workspace_repo(params.workspace_slug, params.repo_slug, context).await?;

        self.client()
            .decline_pull_request(&workspace_slug, &repo_slug, params.pull_request_id)
            .await
            .map_err(|err| map_runtime_error("declining pull request", err))?;

        Ok(format!("Pull request #{} declined.", params.pull_request_id))
    }

    #[tool(
        name = "bitbucket.merge_pull_request",
        description = "Merge a pull request."
    )]
    async fn merge_pull_request(
        &self,
        Parameters(params): Parameters<MergePullRequestParams>,
        context: RequestContext<RoleServer>,
    ) -> Result<String, McpError> {
        let (workspace_slug, repo_slug) = resolve_workspace_repo(params.workspace_slug, params.repo_slug, context).await?;

        self.client()
            .merge_pull_request(
                &workspace_slug,
                &repo_slug,
                params.pull_request_id,
                params.message,
                params.close_source_branch,
                params.merge_strategy.as_deref(),
            )
            .await
            .map_err(|err| map_runtime_error("merging pull request", err))?;

        Ok(format!("Pull request #{} merged.", params.pull_request_id))
    }

    #[tool(
        name = "bitbucket.list_pipelines",
        description = "List pipelines for a repository with optional filtering. When workspace_slug and repo_slug are omitted, the server resolves the repository from the client's MCP roots.",
        annotations(read_only_hint = true)
    )]
    async fn list_pipelines(
        &self,
        Parameters(params): Parameters<ListPipelinesParams>,
        context: RequestContext<RoleServer>,
    ) -> Result<Json<ListPipelinesResult>, McpError> {
        let (workspace_slug, repo_slug) =
            resolve_workspace_repo(params.workspace_slug, params.repo_slug, context).await?;

        let limit = if params.all.unwrap_or(false) { None } else { Some(20) };

        let pipelines = self
            .client()
            .list_pipelines(
                &workspace_slug,
                &repo_slug,
                params.status.as_deref(),
                params.target_branch.as_deref(),
                None,
                limit,
            )
            .await
            .map_err(|err| map_runtime_error("listing pipelines", err))?;

        Ok(Json(ListPipelinesResult {
            pipelines: pipelines.iter().map(PipelineJsonRow::from).collect(),
        }))
    }

    #[tool(
        name = "bitbucket.get_pipeline",
        description = "Get details for a specific pipeline.",
        annotations(read_only_hint = true)
    )]
    async fn get_pipeline(
        &self,
        Parameters(params): Parameters<GetPipelineParams>,
        context: RequestContext<RoleServer>,
    ) -> Result<Json<PipelineDetailedJsonRow>, McpError> {
        let (workspace_slug, repo_slug) =
            resolve_workspace_repo(params.workspace_slug, params.repo_slug, context).await?;

        let pipeline = self
            .client()
            .get_pipeline(&workspace_slug, &repo_slug, &params.pipeline_uuid)
            .await
            .map_err(|err| map_runtime_error("getting pipeline", err))?;

        Ok(Json(PipelineDetailedJsonRow::from(&pipeline)))
    }

    #[tool(
        name = "bitbucket.trigger_pipeline",
        description = "Trigger a new pipeline run. Supports branch/tag triggers, custom selectors, specific commits, and variables."
    )]
    async fn trigger_pipeline(
        &self,
        Parameters(params): Parameters<TriggerPipelineParams>,
        context: RequestContext<RoleServer>,
    ) -> Result<Json<PipelineDetailedJsonRow>, McpError> {
        let (workspace_slug, repo_slug) =
            resolve_workspace_repo(params.workspace_slug, params.repo_slug, context).await?;

        let mut vars = Vec::new();
        if let Some(ref variables) = params.variables {
            for (key, value) in variables {
                vars.push((key.clone(), value.clone(), false));
            }
        }
        if let Some(ref secure_vars) = params.secure_variables {
            for (key, value) in secure_vars {
                vars.push((key.clone(), value.clone(), true));
            }
        }
        let vars_opt = if vars.is_empty() { None } else { Some(vars) };

        let pipeline = self
            .client()
            .trigger_pipeline(
                &workspace_slug,
                &repo_slug,
                &params.ref_type,
                &params.ref_name,
                params.commit_hash.as_deref(),
                params.selector_type.as_deref(),
                params.selector_pattern.as_deref(),
                vars_opt,
            )
            .await
            .map_err(|err| map_runtime_error("triggering pipeline", err))?;

        Ok(Json(PipelineDetailedJsonRow::from(&pipeline)))
    }

    #[tool(name = "bitbucket.stop_pipeline", description = "Stop a running pipeline.")]
    async fn stop_pipeline(
        &self,
        Parameters(params): Parameters<StopPipelineParams>,
        context: RequestContext<RoleServer>,
    ) -> Result<String, McpError> {
        let (workspace_slug, repo_slug) =
            resolve_workspace_repo(params.workspace_slug, params.repo_slug, context).await?;

        self.client()
            .stop_pipeline(&workspace_slug, &repo_slug, &params.pipeline_uuid)
            .await
            .map_err(|err| map_runtime_error("stopping pipeline", err))?;

        Ok("Pipeline stopped.".to_string())
    }

    #[tool(
        name = "bitbucket.list_pipeline_steps",
        description = "List steps for a pipeline.",
        annotations(read_only_hint = true)
    )]
    async fn list_pipeline_steps(
        &self,
        Parameters(params): Parameters<ListPipelineStepsParams>,
        context: RequestContext<RoleServer>,
    ) -> Result<Json<ListPipelineStepsResult>, McpError> {
        let (workspace_slug, repo_slug) =
            resolve_workspace_repo(params.workspace_slug, params.repo_slug, context).await?;

        let steps = self
            .client()
            .list_pipeline_steps(&workspace_slug, &repo_slug, &params.pipeline_uuid)
            .await
            .map_err(|err| map_runtime_error("listing pipeline steps", err))?;

        Ok(Json(ListPipelineStepsResult {
            steps: steps.iter().map(PipelineStepJsonRow::from).collect(),
        }))
    }

    #[tool(
        name = "bitbucket.get_pipeline_step",
        description = "Get details for a specific pipeline step.",
        annotations(read_only_hint = true)
    )]
    async fn get_pipeline_step(
        &self,
        Parameters(params): Parameters<GetPipelineStepParams>,
        context: RequestContext<RoleServer>,
    ) -> Result<Json<PipelineStepJsonRow>, McpError> {
        let (workspace_slug, repo_slug) =
            resolve_workspace_repo(params.workspace_slug, params.repo_slug, context).await?;

        let step = self
            .client()
            .get_pipeline_step(&workspace_slug, &repo_slug, &params.pipeline_uuid, &params.step_uuid)
            .await
            .map_err(|err| map_runtime_error("getting pipeline step", err))?;

        Ok(Json(PipelineStepJsonRow::from(&step)))
    }

    #[tool(
        name = "bitbucket.get_pipeline_step_log",
        description = "Get logs for a specific pipeline step.",
        annotations(read_only_hint = true)
    )]
    async fn get_pipeline_step_log(
        &self,
        Parameters(params): Parameters<GetPipelineStepLogParams>,
        context: RequestContext<RoleServer>,
    ) -> Result<String, McpError> {
        let (workspace_slug, repo_slug) =
            resolve_workspace_repo(params.workspace_slug, params.repo_slug, context).await?;

        let logs = self
            .client()
            .get_pipeline_step_log(&workspace_slug, &repo_slug, &params.pipeline_uuid, &params.step_uuid)
            .await
            .map_err(|err| map_runtime_error("getting pipeline step log", err))?;

        Ok(logs)
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

#[derive(Debug, Deserialize, JsonSchema)]
struct GetPullRequestParams {
    workspace_slug: Option<String>,
    repo_slug: Option<String>,
    pull_request_id: i32,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct CreatePullRequestCommentParams {
    /// The workspace containing the repository.
    workspace_slug: Option<String>,
    /// The repository slug.
    repo_slug: Option<String>,
    /// The pull request ID.
    pull_request_id: i32,
    /// The content of the comment in markdown format.
    content: String,
    /// Optional inline comment details.
    inline: Option<crate::model::PullRequestInlineComment>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct MergePullRequestParams {
    /// The workspace containing the repository.
    workspace_slug: Option<String>,
    /// The repository slug.
    repo_slug: Option<String>,
    /// The pull request ID.
    pull_request_id: i32,
    /// Optional commit message.
    message: Option<String>,
    /// Whether to delete the source branch after merging.
    close_source_branch: Option<bool>,
    /// Optional merge strategy: merge_commit, squash, fast_forward, squash_fast_forward, rebase_fast_forward, rebase_merge.
    merge_strategy: Option<String>,
}

#[derive(Debug, Serialize, JsonSchema)]
struct ListRepositoriesResult {
    repositories: Vec<RepoJsonRow>,
}

#[derive(Debug, Serialize, JsonSchema)]
struct ListPullRequestsResult {
    pull_requests: Vec<PullRequestJsonRow>,
}

#[derive(Debug, Serialize, JsonSchema)]
struct ListPullRequestCommentsResult {
    comments: Vec<PullRequestCommentJsonRow>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct ListPipelinesParams {
    /// Workspace slug (inferred from roots if omitted)
    workspace_slug: Option<String>,
    /// Repository slug (inferred from roots if omitted)
    repo_slug: Option<String>,
    /// Filter by pipeline status (PENDING, IN_PROGRESS, COMPLETED, FAILED, etc.)
    status: Option<String>,
    /// Filter by target branch name
    target_branch: Option<String>,
    /// List all pipelines (default: limit to 20)
    all: Option<bool>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct GetPipelineParams {
    workspace_slug: Option<String>,
    repo_slug: Option<String>,
    /// Pipeline UUID
    pipeline_uuid: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct TriggerPipelineParams {
    workspace_slug: Option<String>,
    repo_slug: Option<String>,
    /// Reference type (branch, tag, named_branch, bookmark)
    ref_type: String,
    /// Reference name (branch or tag name)
    ref_name: String,
    /// Optional specific commit hash
    commit_hash: Option<String>,
    /// Custom pipeline selector type (e.g., "custom")
    selector_type: Option<String>,
    /// Custom pipeline selector pattern
    selector_pattern: Option<String>,
    /// Pipeline variables as key-value pairs
    variables: Option<std::collections::HashMap<String, String>>,
    /// Secured pipeline variables as key-value pairs (values will be hidden in logs)
    secure_variables: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct StopPipelineParams {
    workspace_slug: Option<String>,
    repo_slug: Option<String>,
    /// Pipeline UUID to stop
    pipeline_uuid: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct ListPipelineStepsParams {
    workspace_slug: Option<String>,
    repo_slug: Option<String>,
    /// Pipeline UUID
    pipeline_uuid: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct GetPipelineStepParams {
    workspace_slug: Option<String>,
    repo_slug: Option<String>,
    /// Pipeline UUID
    pipeline_uuid: String,
    /// Step UUID
    step_uuid: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
struct GetPipelineStepLogParams {
    workspace_slug: Option<String>,
    repo_slug: Option<String>,
    /// Pipeline UUID
    pipeline_uuid: String,
    /// Step UUID
    step_uuid: String,
}

#[derive(Debug, Serialize, JsonSchema)]
struct ListPipelinesResult {
    pipelines: Vec<PipelineJsonRow>,
}

#[derive(Debug, Serialize, JsonSchema)]
struct ListPipelineStepsResult {
    steps: Vec<PipelineStepJsonRow>,
}

async fn resolve_workspace_repo(
    workspace_slug: Option<String>,
    repo_slug: Option<String>,
    context: RequestContext<RoleServer>,
) -> Result<(String, String), McpError> {
    match (workspace_slug, repo_slug) {
        (Some(w), Some(r)) => Ok((w, r)),
        (None, None) => resolve_repo_from_context(context).await,
        _ => Err(McpError::invalid_params(
            "workspace_slug and repo_slug must be provided together",
            None,
        )),
    }
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
