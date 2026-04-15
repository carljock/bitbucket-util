use bbapi::apis::{self, configuration::Configuration};
use bbapi::models;
use serde::de::DeserializeOwned;

use crate::error::BbcliError;
use crate::model::{RepoRow, WorkspaceRef, normalize_repositories};

pub struct BitbucketClient {
    config: Configuration,
}

impl BitbucketClient {
    pub fn from_credentials(login: String, token: String) -> Self {
        let mut config = Configuration::new();
        config.user_agent = Some(format!("bb/{}", env!("CARGO_PKG_VERSION")));
        config.basic_auth = Some((login, Some(token)));

        Self { config }
    }

    pub async fn list_accessible_repositories(
        &self,
        workspace_filter: Option<&str>,
        role: Option<&str>,
    ) -> Result<Vec<RepoRow>, BbcliError> {
        let workspaces = match workspace_filter {
            Some(workspace_slug) => vec![WorkspaceRef {
                slug: workspace_slug.to_owned(),
                uuid: None,
            }],
            None => self.list_workspaces().await?,
        };

        let mut repos = Vec::new();
        for workspace in workspaces {
            repos.extend(
                self.list_repositories_in_workspace(workspace.slug.as_str(), role)
                    .await?,
            );
        }

        Ok(normalize_repositories(repos))
    }

    pub async fn list_workspaces(&self) -> Result<Vec<WorkspaceRef>, BbcliError> {
        let mut page = apis::workspaces_api::user_workspaces_get(&self.config, None, None)
            .await
            .map_err(|err| map_sdk_error("list workspaces", err))?;

        let mut workspaces = workspace_refs_from_values(page.values.take().unwrap_or_default());
        let mut next = page.next.take();

        while let Some(next_url) = next {
            let mut next_page: models::PaginatedWorkspaceAccess = self
                .fetch_page(next_url.as_str(), "list workspaces pagination")
                .await?;

            workspaces.extend(workspace_refs_from_values(
                next_page.values.take().unwrap_or_default(),
            ));
            next = next_page.next.take();
        }

        workspaces.sort_by(|a, b| a.slug.cmp(&b.slug));
        workspaces.dedup_by(|a, b| a.slug == b.slug);

        Ok(workspaces)
    }

    pub async fn list_repositories_in_workspace(
        &self,
        workspace_slug: &str,
        role: Option<&str>,
    ) -> Result<Vec<RepoRow>, BbcliError> {
        let mut page = apis::repositories_api::repositories_workspace_get(
            &self.config,
            workspace_slug,
            role,
            None,
            None,
        )
        .await
        .map_err(|err| map_sdk_error("list repositories", err))?;

        let mut repos =
            repo_rows_from_values(workspace_slug, page.values.take().unwrap_or_default());
        let mut next = page.next.take();

        while let Some(next_url) = next {
            let mut next_page: models::PaginatedRepositories = self
                .fetch_page(next_url.as_str(), "list repositories pagination")
                .await?;

            repos.extend(repo_rows_from_values(
                workspace_slug,
                next_page.values.take().unwrap_or_default(),
            ));
            next = next_page.next.take();
        }

        Ok(repos)
    }

    async fn fetch_page<T: DeserializeOwned>(
        &self,
        url: &str,
        context: &'static str,
    ) -> Result<T, BbcliError> {
        let mut request = self.config.client.get(url);

        if let Some(user_agent) = &self.config.user_agent {
            request = request.header("User-Agent", user_agent.clone());
        }

        if let Some((username, password)) = &self.config.basic_auth {
            request = request.basic_auth(username, password.clone());
        }

        let response = request.send().await.map_err(|err| BbcliError::Network {
            context,
            message: err.to_string(),
        })?;

        let status = response.status();
        let body = response.text().await.map_err(|err| BbcliError::Network {
            context,
            message: err.to_string(),
        })?;

        if !status.is_success() {
            return Err(BbcliError::Api {
                context,
                status: Some(status.as_u16()),
                message: compact_api_message(body.as_str()),
            });
        }

        serde_json::from_str(body.as_str()).map_err(|err| BbcliError::Parse {
            context,
            message: err.to_string(),
        })
    }
}

fn workspace_refs_from_values(values: Vec<models::WorkspaceAccess>) -> Vec<WorkspaceRef> {
    let mut workspaces = Vec::new();

    for value in values {
        let Some(workspace) = value.workspace else {
            continue;
        };

        let workspace = *workspace;
        let Some(slug) = workspace.slug else {
            continue;
        };

        workspaces.push(WorkspaceRef {
            slug,
            uuid: workspace.uuid,
        });
    }

    workspaces
}

fn repo_rows_from_values(workspace_slug: &str, values: Vec<models::Repository>) -> Vec<RepoRow> {
    let mut repos = Vec::new();

    for repository in values {
        if let Some(row) = repository_to_repo_row(workspace_slug, repository) {
            repos.push(row);
        }
    }

    repos
}

fn repository_to_repo_row(workspace_slug: &str, repository: models::Repository) -> Option<RepoRow> {
    let full_name = repository.full_name;
    let name = repository.name;
    let description = repository.description;
    let is_private = repository.is_private;
    let language = repository.language;
    let updated_on = repository.updated_on;
    let main_branch = repository.mainbranch.and_then(|branch| branch.name);
    let (clone_https, clone_ssh) = extract_clone_urls(repository.links);

    let repo_slug = derive_repo_slug(
        full_name.as_deref(),
        name.as_deref(),
        repository.uuid.as_deref(),
    )?;

    Some(RepoRow {
        workspace_slug: workspace_slug.to_owned(),
        repo_slug,
        full_name,
        name,
        description,
        is_private,
        language,
        updated_on,
        main_branch,
        clone_https,
        clone_ssh,
    })
}

fn extract_clone_urls(
    links: Option<Box<models::RepositoryLinks>>,
) -> (Option<String>, Option<String>) {
    let mut clone_https = None;
    let mut clone_ssh = None;

    if let Some(links) = links {
        if let Some(clone_links) = links.clone {
            for link in clone_links {
                match link.name.as_deref() {
                    Some("https") => clone_https = link.href,
                    Some("ssh") => clone_ssh = link.href,
                    _ => {}
                }
            }
        }
    }

    (clone_https, clone_ssh)
}

fn derive_repo_slug(
    full_name: Option<&str>,
    name: Option<&str>,
    uuid: Option<&str>,
) -> Option<String> {
    if let Some(full_name) = full_name {
        if let Some((_, slug)) = full_name.rsplit_once('/') {
            if !slug.trim().is_empty() {
                return Some(slug.to_owned());
            }
        }
    }

    if let Some(name) = name {
        if !name.trim().is_empty() {
            return Some(name.to_owned());
        }
    }

    if let Some(uuid) = uuid {
        let trimmed = uuid.trim_matches(['{', '}']);
        if !trimmed.is_empty() {
            return Some(trimmed.to_owned());
        }
    }

    None
}

fn compact_api_message(message: &str) -> String {
    let trimmed = message.trim();
    if trimmed.is_empty() {
        return "request failed with an empty response body".to_string();
    }

    const MAX_LEN: usize = 300;
    if trimmed.len() > MAX_LEN {
        format!("{}...", &trimmed[..MAX_LEN])
    } else {
        trimmed.to_string()
    }
}

fn map_sdk_error<T: std::fmt::Debug>(context: &'static str, error: apis::Error<T>) -> BbcliError {
    match error {
        apis::Error::Reqwest(err) => BbcliError::Network {
            context,
            message: err.to_string(),
        },
        apis::Error::Serde(err) => BbcliError::Parse {
            context,
            message: err.to_string(),
        },
        apis::Error::Io(source) => BbcliError::with_io(context, source),
        apis::Error::ResponseError(response) => BbcliError::Api {
            context,
            status: Some(response.status.as_u16()),
            message: compact_api_message(response.content.as_str()),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{derive_repo_slug, extract_clone_urls, repository_to_repo_row};
    use bbapi::models;

    #[test]
    fn derive_repo_slug_prefers_full_name_suffix() {
        let slug = derive_repo_slug(Some("acme/service"), Some("ignored"), None)
            .expect("slug should be present");
        assert_eq!(slug, "service");
    }

    #[test]
    fn derive_repo_slug_falls_back_to_name() {
        let slug =
            derive_repo_slug(None, Some("service"), None).expect("slug should fall back to name");
        assert_eq!(slug, "service");
    }

    #[test]
    fn derive_repo_slug_falls_back_to_uuid() {
        let slug =
            derive_repo_slug(None, None, Some("{abc-123}")).expect("slug should fall back to uuid");
        assert_eq!(slug, "abc-123");
    }

    #[test]
    fn workspace_access_deserializes_boolean_administrator() {
        let payload = r#"
        {
          "values": [
            {
              "type": "workspace_access",
              "administrator": false,
              "workspace": {
                "type": "workspace_base",
                "slug": "acme",
                "uuid": "{abc}"
              }
            }
          ]
        }"#;

        let page: models::PaginatedWorkspaceAccess =
            serde_json::from_str(payload).expect("payload should deserialize");

        let values = page.values.expect("values should be present");
        assert_eq!(values.len(), 1);
        assert_eq!(values[0].administrator, Some(false));
    }

    #[test]
    fn extract_clone_urls_picks_https_and_ssh() {
        let links = models::RepositoryLinks {
            clone: Some(vec![
                models::Link1 {
                    href: Some("https://bitbucket.org/acme/repo.git".into()),
                    name: Some("https".into()),
                },
                models::Link1 {
                    href: Some("git@bitbucket.org:acme/repo.git".into()),
                    name: Some("ssh".into()),
                },
            ]),
            ..Default::default()
        };

        let (https, ssh) = extract_clone_urls(Some(Box::new(links)));
        assert_eq!(https.as_deref(), Some("https://bitbucket.org/acme/repo.git"));
        assert_eq!(ssh.as_deref(), Some("git@bitbucket.org:acme/repo.git"));
    }

    #[test]
    fn repository_to_repo_row_maps_extended_json_fields() {
        let repository = models::Repository {
            r#type: "repository".into(),
            uuid: Some("{abc}".into()),
            full_name: Some("acme/repo".into()),
            name: Some("repo".into()),
            description: Some("desc".into()),
            is_private: Some(false),
            language: Some("rust".into()),
            updated_on: Some("2026-04-15T12:00:00+00:00".into()),
            mainbranch: Some(Box::new(models::Branch {
                r#type: "branch".into(),
                name: Some("main".into()),
                ..Default::default()
            })),
            links: Some(Box::new(models::RepositoryLinks {
                clone: Some(vec![
                    models::Link1 {
                        href: Some("https://bitbucket.org/acme/repo.git".into()),
                        name: Some("https".into()),
                    },
                    models::Link1 {
                        href: Some("git@bitbucket.org:acme/repo.git".into()),
                        name: Some("ssh".into()),
                    },
                ]),
                ..Default::default()
            })),
            ..Default::default()
        };

        let row = repository_to_repo_row("acme", repository).expect("row should be created");
        assert_eq!(row.language.as_deref(), Some("rust"));
        assert_eq!(row.updated_on.as_deref(), Some("2026-04-15T12:00:00+00:00"));
        assert_eq!(row.main_branch.as_deref(), Some("main"));
        assert_eq!(row.clone_https.as_deref(), Some("https://bitbucket.org/acme/repo.git"));
        assert_eq!(row.clone_ssh.as_deref(), Some("git@bitbucket.org:acme/repo.git"));
    }
}
