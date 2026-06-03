use bbapi::apis::{self, configuration::Configuration};
use bbapi::models;
use serde::de::DeserializeOwned;

use crate::error::BbcliError;
use crate::model::{PullRequestRow, RepoRow, WorkspaceRef, normalize_repositories};

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

    #[cfg(test)]
    fn from_configuration(config: Configuration) -> Self {
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

    pub async fn list_pull_requests_in_repository(
        &self,
        workspace_slug: &str,
        repo_slug: &str,
        state: Option<&str>,
    ) -> Result<Vec<crate::model::PullRequestRow>, BbcliError> {
        let mut page = apis::pullrequests_api::repositories_workspace_repo_slug_pullrequests_get(
            &self.config,
            repo_slug,
            workspace_slug,
            state,
        )
        .await
        .map_err(|err| map_sdk_error("list pull requests", err))?;

        let mut pull_requests = pull_request_rows_from_values(
            workspace_slug,
            repo_slug,
            page.values.take().unwrap_or_default(),
        );
        let mut next = page.next.take();

        while let Some(next_url) = next {
            let mut next_page: models::PaginatedPullrequests = self
                .fetch_page(next_url.as_str(), "list pull requests pagination")
                .await?;

            pull_requests.extend(pull_request_rows_from_values(
                workspace_slug,
                repo_slug,
                next_page.values.take().unwrap_or_default(),
            ));
            next = next_page.next.take();
        }

        Ok(pull_requests)
    }

    pub async fn get_pull_request(
        &self,
        workspace_slug: &str,
        repo_slug: &str,
        id: i32,
    ) -> Result<crate::model::PullRequestDetailedRow, BbcliError> {
        let pr = apis::pullrequests_api::repositories_workspace_repo_slug_pullrequests_pull_request_id_get(
            &self.config,
            id,
            repo_slug,
            workspace_slug,
        )
        .await
        .map_err(|err| map_sdk_error("get pull request", err))?;

        Ok(pull_request_to_detailed_row(workspace_slug, repo_slug, pr))
    }

    pub async fn get_pull_request_diff(
        &self,
        workspace_slug: &str,
        repo_slug: &str,
        id: i32,
    ) -> Result<String, BbcliError> {
        let mut request = self.config.client.get(format!(
            "{}/repositories/{workspace_slug}/{repo_slug}/pullrequests/{id}/diff",
            self.config.base_path
        ));

        if let Some(user_agent) = &self.config.user_agent {
            request = request.header("User-Agent", user_agent.clone());
        }

        if let Some((username, password)) = &self.config.basic_auth {
            request = request.basic_auth(username, password.clone());
        }

        let response = request.send().await.map_err(|err| BbcliError::Network {
            context: "get pull request diff",
            message: err.to_string(),
        })?;

        let status = response.status();
        let body = response.text().await.map_err(|err| BbcliError::Network {
            context: "get pull request diff",
            message: err.to_string(),
        })?;

        if !status.is_success() {
            return Err(BbcliError::Api {
                context: "get pull request diff",
                status: Some(status.as_u16()),
                message: compact_api_message(body.as_str()),
            });
        }

        Ok(body)
    }

    pub async fn list_pull_request_comments(
        &self,
        workspace_slug: &str,
        repo_slug: &str,
        id: i32,
    ) -> Result<Vec<crate::model::PullRequestCommentRow>, BbcliError> {
        let mut page = apis::pullrequests_api::repositories_workspace_repo_slug_pullrequests_pull_request_id_comments_get(
            &self.config,
            id,
            repo_slug,
            workspace_slug,
        )
        .await
        .map_err(|err| map_sdk_error("list pull request comments", err))?;

        let mut comments = pull_request_comment_rows_from_values(page.values.take().unwrap_or_default());
        let mut next = page.next.take();

        while let Some(next_url) = next {
            let mut next_page: models::PaginatedPullrequestComments = self
                .fetch_page(next_url.as_str(), "list pull request comments pagination")
                .await?;

            comments.extend(pull_request_comment_rows_from_values(
                next_page.values.take().unwrap_or_default(),
            ));
            next = next_page.next.take();
        }

        Ok(comments)
    }

    pub async fn create_pull_request_comment(
        &self,
        workspace_slug: &str,
        repo_slug: &str,
        id: i32,
        content: &str,
        inline: Option<crate::model::PullRequestInlineComment>,
    ) -> Result<crate::model::PullRequestCommentRow, BbcliError> {
        let mut comment = models::PullrequestComment::new();
        comment.content = Some(Box::new(models::CommentContent {
            raw: Some(content.to_owned()),
            ..Default::default()
        }));

        if let Some(inline_params) = inline {
            comment.inline = Some(Box::new(models::CommentInline {
                path: inline_params.path,
                from: inline_params.from,
                to: inline_params.to,
                ..Default::default()
            }));
        }

        let created = apis::pullrequests_api::repositories_workspace_repo_slug_pullrequests_pull_request_id_comments_post(
            &self.config,
            id,
            repo_slug,
            workspace_slug,
            comment,
        )
        .await
        .map_err(|err| map_sdk_error("create pull request comment", err))?;

        Ok(pull_request_comment_to_row(created))
    }

    pub async fn approve_pull_request(
        &self,
        workspace_slug: &str,
        repo_slug: &str,
        id: i32,
    ) -> Result<(), BbcliError> {
        apis::pullrequests_api::repositories_workspace_repo_slug_pullrequests_pull_request_id_approve_post(
            &self.config,
            id,
            repo_slug,
            workspace_slug,
        )
        .await
        .map_err(|err| map_sdk_error("approve pull request", err))?;

        Ok(())
    }

    pub async fn unapprove_pull_request(
        &self,
        workspace_slug: &str,
        repo_slug: &str,
        id: i32,
    ) -> Result<(), BbcliError> {
        apis::pullrequests_api::repositories_workspace_repo_slug_pullrequests_pull_request_id_approve_delete(
            &self.config,
            id,
            repo_slug,
            workspace_slug,
        )
        .await
        .map_err(|err| map_sdk_error("unapprove pull request", err))?;

        Ok(())
    }

    pub async fn decline_pull_request(
        &self,
        workspace_slug: &str,
        repo_slug: &str,
        id: i32,
    ) -> Result<(), BbcliError> {
        apis::pullrequests_api::repositories_workspace_repo_slug_pullrequests_pull_request_id_decline_post(
            &self.config,
            id,
            repo_slug,
            workspace_slug,
        )
        .await
        .map_err(|err| map_sdk_error("decline pull request", err))?;

        Ok(())
    }

    pub async fn merge_pull_request(
        &self,
        workspace_slug: &str,
        repo_slug: &str,
        id: i32,
        message: Option<String>,
        close_source_branch: Option<bool>,
        merge_strategy: Option<&str>,
    ) -> Result<(), BbcliError> {
        let mut params = models::PullrequestMergeParameters::new("pullrequest_merge_parameters".to_owned());
        params.message = message;
        params.close_source_branch = close_source_branch;
        params.merge_strategy = merge_strategy.and_then(|s| match s {
            "merge_commit" => Some(models::pullrequest_merge_parameters::MergeStrategy::MergeCommit),
            "squash" => Some(models::pullrequest_merge_parameters::MergeStrategy::Squash),
            "fast_forward" => Some(models::pullrequest_merge_parameters::MergeStrategy::FastForward),
            "squash_fast_forward" => Some(models::pullrequest_merge_parameters::MergeStrategy::SquashFastForward),
            "rebase_fast_forward" => Some(models::pullrequest_merge_parameters::MergeStrategy::RebaseFastForward),
            "rebase_merge" => Some(models::pullrequest_merge_parameters::MergeStrategy::RebaseMerge),
            _ => None,
        });

        apis::pullrequests_api::repositories_workspace_repo_slug_pullrequests_pull_request_id_merge_post(
            &self.config,
            id,
            repo_slug,
            workspace_slug,
            None,
            Some(params),
        )
        .await
        .map_err(|err| map_sdk_error("merge pull request", err))?;

        Ok(())
    }

    pub async fn list_pipelines(
        &self,
        workspace_slug: &str,
        repo_slug: &str,
        status: Option<&str>,
        target_branch: Option<&str>,
        creator_uuid: Option<&str>,
        limit: Option<i32>,
    ) -> Result<Vec<crate::model::PipelineRow>, BbcliError> {
        let limit = limit.unwrap_or(20);
        let mut page = apis::pipelines_api::get_pipelines_for_repository(
            &self.config,
            workspace_slug,
            repo_slug,
            creator_uuid,
            None,
            None,
            target_branch,
            None,
            None,
            None,
            None,
            None,
            status,
            None,
            None,
            Some(limit),
        )
        .await
        .map_err(|err| map_sdk_error("list pipelines", err))?;

        let mut pipelines = pipeline_rows_from_values(page.values.take().unwrap_or_default());
        let mut next = page.next.take();

        while let Some(next_url) = next {
            if pipelines.len() >= limit as usize {
                break;
            }

            let mut next_page: models::PaginatedPipelines = self
                .fetch_page(next_url.as_str(), "list pipelines pagination")
                .await?;

            pipelines.extend(pipeline_rows_from_values(
                next_page.values.take().unwrap_or_default(),
            ));
            next = next_page.next.take();
        }

        pipelines.truncate(limit as usize);
        Ok(pipelines)
    }

    pub async fn get_pipeline(
        &self,
        workspace_slug: &str,
        repo_slug: &str,
        pipeline_uuid: &str,
    ) -> Result<crate::model::PipelineDetailedRow, BbcliError> {
        let pipeline = apis::pipelines_api::get_pipeline_for_repository(
            &self.config,
            workspace_slug,
            repo_slug,
            pipeline_uuid,
        )
        .await
        .map_err(|err| map_sdk_error("get pipeline", err))?;

        Ok(pipeline_to_detailed_row(pipeline))
    }

    pub async fn trigger_pipeline(
        &self,
        workspace_slug: &str,
        repo_slug: &str,
        ref_type: &str,
        ref_name: &str,
        commit_hash: Option<&str>,
        selector_type: Option<&str>,
        selector_pattern: Option<&str>,
        variables: Option<Vec<(String, String, bool)>>,
    ) -> Result<crate::model::PipelineDetailedRow, BbcliError> {
        // Build the request JSON with proper discriminated union structure
        let mut target_json = serde_json::json!({
            "type": "pipeline_ref_target",
            "ref_type": ref_type,
            "ref_name": ref_name,
        });

        if let Some(commit) = commit_hash {
            target_json["commit"] = serde_json::json!({
                "type": "commit",
                "hash": commit,
            });
        }

        if selector_type.is_some() || selector_pattern.is_some() {
            let mut selector_obj = serde_json::json!({});
            if let Some(sel_type) = selector_type {
                selector_obj["type"] = serde_json::Value::String(sel_type.to_string());
            }
            if let Some(pattern) = selector_pattern {
                selector_obj["pattern"] = serde_json::Value::String(pattern.to_string());
            }
            target_json["selector"] = selector_obj;
        }

        // Deserialize to PipelineRefTarget to validate the structure
        let _target: models::PipelineRefTarget = serde_json::from_value(target_json.clone())
            .map_err(|e| BbcliError::Parse {
                context: "parse pipeline ref target",
                message: e.to_string(),
            })?;

        // Now build the Pipeline request with the JSON target
        // (We'll send raw JSON instead of using SDK structures for now)
        let mut pipeline_json = serde_json::json!({
            "type": "pipeline",
            "target": target_json,
        });

        if let Some(vars) = variables {
            let vars_json: Vec<_> = vars.iter()
                .map(|(key, value, secured)| {
                    serde_json::json!({
                        "type": "pipeline_variable",
                        "key": key,
                        "value": value,
                        "secured": secured,
                    })
                })
                .collect();
            pipeline_json["variables"] = serde_json::Value::Array(vars_json);
        }

        // Send the request directly with raw JSON
        let uri = format!(
            "{}/repositories/{}/{}/pipelines",
            self.config.base_path,
            apis::urlencode(workspace_slug),
            apis::urlencode(repo_slug)
        );

        let mut req_builder = self.config.client.post(&uri);

        if let Some(user_agent) = &self.config.user_agent {
            req_builder = req_builder.header("User-Agent", user_agent.clone());
        }

        if let Some((username, password)) = &self.config.basic_auth {
            req_builder = req_builder.basic_auth(username, password.clone());
        }

        let req = req_builder
            .json(&pipeline_json)
            .build()
            .map_err(|e| BbcliError::Network {
                context: "build trigger pipeline request",
                message: e.to_string(),
            })?;

        let resp = self.config.client.execute(req).await.map_err(|e| {
            BbcliError::Network {
                context: "execute trigger pipeline request",
                message: e.to_string(),
            }
        })?;

        let status = resp.status();
        let body = resp.text().await.map_err(|e| BbcliError::Network {
            context: "read trigger pipeline response",
            message: e.to_string(),
        })?;

        if !status.is_success() {
            return Err(BbcliError::Api {
                context: "trigger pipeline",
                status: Some(status.as_u16()),
                message: compact_api_message(&body),
            });
        }

        let pipeline: models::Pipeline = serde_json::from_str(&body).map_err(|e| {
            BbcliError::Parse {
                context: "parse trigger pipeline response",
                message: e.to_string(),
            }
        })?;

        Ok(pipeline_to_detailed_row(pipeline))
    }

    pub async fn stop_pipeline(
        &self,
        workspace_slug: &str,
        repo_slug: &str,
        pipeline_uuid: &str,
    ) -> Result<(), BbcliError> {
        apis::pipelines_api::stop_pipeline(
            &self.config,
            workspace_slug,
            repo_slug,
            pipeline_uuid,
        )
        .await
        .map_err(|err| map_sdk_error("stop pipeline", err))?;

        Ok(())
    }

    pub async fn list_pipeline_steps(
        &self,
        workspace_slug: &str,
        repo_slug: &str,
        pipeline_uuid: &str,
    ) -> Result<Vec<crate::model::PipelineStepRow>, BbcliError> {
        let mut page = apis::pipelines_api::get_pipeline_steps_for_repository(
            &self.config,
            workspace_slug,
            repo_slug,
            pipeline_uuid,
        )
        .await
        .map_err(|err| map_sdk_error("list pipeline steps", err))?;

        let mut steps = pipeline_step_rows_from_values(page.values.take().unwrap_or_default());
        let mut next = page.next.take();

        while let Some(next_url) = next {
            let mut next_page: models::PaginatedPipelineSteps = self
                .fetch_page(next_url.as_str(), "list pipeline steps pagination")
                .await?;

            steps.extend(pipeline_step_rows_from_values(
                next_page.values.take().unwrap_or_default(),
            ));
            next = next_page.next.take();
        }

        Ok(steps)
    }

    pub async fn get_pipeline_step(
        &self,
        workspace_slug: &str,
        repo_slug: &str,
        pipeline_uuid: &str,
        step_uuid: &str,
    ) -> Result<crate::model::PipelineStepRow, BbcliError> {
        let step = apis::pipelines_api::get_pipeline_step_for_repository(
            &self.config,
            workspace_slug,
            repo_slug,
            pipeline_uuid,
            step_uuid,
        )
        .await
        .map_err(|err| map_sdk_error("get pipeline step", err))?;

        Ok(pipeline_step_to_row(step))
    }

    pub async fn get_pipeline_step_log(
        &self,
        workspace_slug: &str,
        repo_slug: &str,
        pipeline_uuid: &str,
        step_uuid: &str,
    ) -> Result<String, BbcliError> {
        let mut request = self.config.client.get(format!(
            "{}/repositories/{workspace_slug}/{repo_slug}/pipelines/{pipeline_uuid}/steps/{step_uuid}/log",
            self.config.base_path
        ));

        if let Some(user_agent) = &self.config.user_agent {
            request = request.header("User-Agent", user_agent.clone());
        }

        if let Some((username, password)) = &self.config.basic_auth {
            request = request.basic_auth(username, password.clone());
        }

        let response = request.send().await.map_err(|err| BbcliError::Network {
            context: "get pipeline step log",
            message: err.to_string(),
        })?;

        let status = response.status();
        let body = response.text().await.map_err(|err| BbcliError::Network {
            context: "get pipeline step log",
            message: err.to_string(),
        })?;

        if !status.is_success() {
            return Err(BbcliError::Api {
                context: "get pipeline step log",
                status: Some(status.as_u16()),
                message: compact_api_message(body.as_str()),
            });
        }

        Ok(body)
    }
}

fn pull_request_comment_rows_from_values(
    values: Vec<models::PullrequestComment>,
) -> Vec<crate::model::PullRequestCommentRow> {
    values.into_iter().map(pull_request_comment_to_row).collect()
}

fn pull_request_comment_to_row(comment: models::PullrequestComment) -> crate::model::PullRequestCommentRow {
    crate::model::PullRequestCommentRow {
        id: comment.id.unwrap_or(0),
        author_display_name: comment.user.and_then(|u| u.display_name),
        content_raw: comment.content.and_then(|c| c.raw),
        inline: comment.inline.map(|i| crate::model::PullRequestInlineComment {
            path: i.path,
            from: i.from,
            to: i.to,
        }),
        created_on: comment.created_on.map(|dt| dt.to_string()),
        updated_on: comment.updated_on.map(|dt| dt.to_string()),
    }
}

fn pull_request_to_detailed_row(
    workspace_slug: &str,
    repo_slug: &str,
    pr: models::Pullrequest,
) -> crate::model::PullRequestDetailedRow {
    let common = pull_request_to_row(workspace_slug, repo_slug, pr.clone()).unwrap();
    let description = pr.summary.and_then(|s| s.raw);
    let reviewers = pr
        .reviewers
        .unwrap_or_default()
        .into_iter()
        .filter_map(|r| r.display_name)
        .collect();

    crate::model::PullRequestDetailedRow {
        common,
        description,
        reviewers,
    }
}

impl BitbucketClient {
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

fn pull_request_rows_from_values(
    workspace_slug: &str,
    repo_slug: &str,
    values: Vec<models::Pullrequest>,
) -> Vec<PullRequestRow> {
    let mut pull_requests = Vec::new();

    for pull_request in values {
        if let Some(row) = pull_request_to_row(workspace_slug, repo_slug, pull_request) {
            pull_requests.push(row);
        }
    }

    pull_requests
}

fn repository_to_repo_row(workspace_slug: &str, repository: models::Repository) -> Option<RepoRow> {
    let full_name = repository.full_name;
    let name = repository.name;
    let description = repository.description;
    let is_private = repository.is_private;
    let language = repository.language;
    let updated_on = repository.updated_on.map(|dt| dt.to_string());
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

fn pull_request_to_row(
    workspace_slug: &str,
    repo_slug: &str,
    pull_request: models::Pullrequest,
) -> Option<PullRequestRow> {
    let id = pull_request.id?;

    Some(PullRequestRow {
        workspace_slug: workspace_slug.to_owned(),
        repo_slug: repo_slug.to_owned(),
        id,
        title: pull_request.title,
        state: pull_request_state_label(pull_request.state),
        html_url: pull_request
            .links
            .and_then(|links| links.html)
            .and_then(|link| link.href),
        author_display_name: pull_request.author.and_then(|account| account.display_name),
        source_branch: pull_request
            .source
            .and_then(|endpoint| endpoint.branch)
            .and_then(|branch| branch.name),
        destination_branch: pull_request
            .destination
            .and_then(|endpoint| endpoint.branch)
            .and_then(|branch| branch.name),
        draft: pull_request.draft,
        comment_count: pull_request.comment_count,
        task_count: pull_request.task_count,
        created_on: pull_request.created_on.map(|dt| dt.to_string()),
        updated_on: pull_request.updated_on.map(|dt| dt.to_string()),
    })
}

fn pull_request_state_label(state: Option<models::pullrequest::State>) -> String {
    match state {
        Some(models::pullrequest::State::Open) => "OPEN",
        Some(models::pullrequest::State::Draft) => "DRAFT",
        Some(models::pullrequest::State::Queued) => "QUEUED",
        Some(models::pullrequest::State::Merged) => "MERGED",
        Some(models::pullrequest::State::Declined) => "DECLINED",
        Some(models::pullrequest::State::Superseded) => "SUPERSEDED",
        None => "UNKNOWN",
    }
    .to_owned()
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

fn pipeline_to_row(pipeline: models::Pipeline) -> Option<crate::model::PipelineRow> {
    let uuid = pipeline.uuid?;
    let build_number = pipeline.build_number?;

    let state = pipeline
        .state
        .and_then(|s| s.r#type)
        .unwrap_or_else(|| "UNKNOWN".to_string());

    let creator = pipeline.creator.and_then(|a| a.display_name);

    let (target_ref_name, target_ref_type) = pipeline
        .target
        .as_ref()
        .and_then(|target| {
            if let Ok(pipeline_ref_target) = serde_json::from_value::<models::PipelineRefTarget>(
                serde_json::to_value(target).ok()?,
            ) {
                // Match on the enum variant to extract fields
                match pipeline_ref_target {
                    models::PipelineRefTarget::PipelineRefTarget {
                        ref_name,
                        ref_type,
                        ..
                    } => {
                        Some((
                            ref_name,
                            ref_type.map(|rt| match rt {
                                models::pipeline_ref_target::RefType::Branch => "branch",
                                models::pipeline_ref_target::RefType::Tag => "tag",
                                models::pipeline_ref_target::RefType::NamedBranch => "named_branch",
                                models::pipeline_ref_target::RefType::Bookmark => "bookmark",
                            }),
                        ))
                    }
                    _ => None,
                }
            } else {
                None
            }
        })
        .unwrap_or((None, None));

    let web_url = pipeline
        .links
        .and_then(|links| links.param_self)
        .and_then(|link| link.href)
        .unwrap_or_default();

    Some(crate::model::PipelineRow {
        uuid,
        build_number,
        state,
        creator,
        created_on: pipeline.created_on.map(|dt| dt.to_string()),
        completed_on: pipeline.completed_on.map(|dt| dt.to_string()),
        target_ref_name,
        target_ref_type: target_ref_type.map(|s| s.to_string()),
        web_url,
    })
}

fn pipeline_to_detailed_row(pipeline: models::Pipeline) -> crate::model::PipelineDetailedRow {
    let uuid = pipeline.uuid.clone().unwrap_or_default();
    let build_number = pipeline.build_number.unwrap_or_default();

    let state = pipeline
        .state
        .as_ref()
        .and_then(|s| s.r#type.as_ref())
        .cloned()
        .unwrap_or_else(|| "UNKNOWN".to_string());

    let creator = pipeline.creator.as_ref().and_then(|a| a.display_name.clone());

    let (target_ref_name, target_ref_type) = pipeline
        .target
        .as_ref()
        .and_then(|target| {
            if let Ok(pipeline_ref_target) = serde_json::from_value::<models::PipelineRefTarget>(
                serde_json::to_value(target).ok()?,
            ) {
                // Match on the enum variant to extract fields
                match pipeline_ref_target {
                    models::PipelineRefTarget::PipelineRefTarget {
                        ref_name,
                        ref_type,
                        ..
                    } => {
                        Some((
                            ref_name,
                            ref_type.map(|rt| match rt {
                                models::pipeline_ref_target::RefType::Branch => "branch",
                                models::pipeline_ref_target::RefType::Tag => "tag",
                                models::pipeline_ref_target::RefType::NamedBranch => "named_branch",
                                models::pipeline_ref_target::RefType::Bookmark => "bookmark",
                            }),
                        ))
                    }
                    _ => None,
                }
            } else {
                None
            }
        })
        .unwrap_or((None, None));

    let web_url = pipeline
        .links
        .as_ref()
        .and_then(|links| links.param_self.as_ref())
        .and_then(|link| link.href.as_ref())
        .cloned()
        .unwrap_or_default();

    let trigger_type = pipeline.trigger.as_ref().and_then(|t| t.r#type.clone());

    let commit_hash = pipeline
        .target
        .as_ref()
        .and_then(|target| {
            if let Ok(pipeline_ref_target) = serde_json::from_value::<models::PipelineRefTarget>(
                serde_json::to_value(target).ok()?,
            ) {
                match pipeline_ref_target {
                    models::PipelineRefTarget::PipelineRefTarget { commit, .. } => {
                        commit.and_then(|c| c.hash)
                    }
                    _ => None,
                }
            } else {
                None
            }
        });

    crate::model::PipelineDetailedRow {
        uuid,
        build_number,
        state,
        creator,
        created_on: pipeline.created_on.map(|dt| dt.to_string()),
        completed_on: pipeline.completed_on.map(|dt| dt.to_string()),
        target_ref_name,
        target_ref_type: target_ref_type.map(|s| s.to_string()),
        web_url,
        trigger_type,
        build_seconds_used: pipeline.build_seconds_used,
        commit_hash,
    }
}

fn pipeline_rows_from_values(values: Vec<models::Pipeline>) -> Vec<crate::model::PipelineRow> {
    values.into_iter().filter_map(pipeline_to_row).collect()
}

fn pipeline_step_to_row(step: models::PipelineStep) -> crate::model::PipelineStepRow {
    let uuid = step.uuid.unwrap_or_default();
    let state = step
        .state
        .as_ref()
        .and_then(|s| s.r#type.as_ref())
        .cloned()
        .unwrap_or_else(|| "UNKNOWN".to_string());

    let image_name = step
        .image
        .as_ref()
        .and_then(|img| img.name.as_ref())
        .cloned();

    crate::model::PipelineStepRow {
        uuid,
        state,
        started_on: step.started_on.map(|dt| dt.to_string()),
        completed_on: step.completed_on.map(|dt| dt.to_string()),
        image_name,
    }
}

fn pipeline_step_rows_from_values(values: Vec<models::PipelineStep>) -> Vec<crate::model::PipelineStepRow> {
    values.into_iter().map(pipeline_step_to_row).collect()
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
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::sync::{Arc, Mutex};
    use std::thread;

    use super::{
        BitbucketClient, derive_repo_slug, extract_clone_urls, pull_request_to_row,
        repository_to_repo_row,
    };
    use bbapi::apis::configuration::Configuration;
    use bbapi::models;
    use tokio::runtime::Runtime;

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
        assert_eq!(
            https.as_deref(),
            Some("https://bitbucket.org/acme/repo.git")
        );
        assert_eq!(ssh.as_deref(), Some("git@bitbucket.org:acme/repo.git"));
    }

    #[test]
    fn repository_to_repo_row_maps_extended_json_fields() {
        let repository = models::Repository {
            r#type: Some("repository".into()),
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
        assert_eq!(
            row.clone_https.as_deref(),
            Some("https://bitbucket.org/acme/repo.git")
        );
        assert_eq!(
            row.clone_ssh.as_deref(),
            Some("git@bitbucket.org:acme/repo.git")
        );
    }

    #[test]
    fn pull_request_to_row_maps_extended_json_fields() {
        let pull_request = models::Pullrequest {
            r#type: Some("pullrequest".into()),
            id: Some(42),
            title: Some("Improve auth".into()),
            state: Some(models::pullrequest::State::Open),
            links: Some(Box::new(models::PullRequestLinks {
                html: Some(Box::new(models::Link1 {
                    href: Some("https://bitbucket.org/acme/api/pull-requests/42".into()),
                    ..Default::default()
                })),
                ..Default::default()
            })),
            author: Some(Box::new(models::Account {
                r#type: Some("account".into()),
                display_name: Some("Alice".into()),
                ..Default::default()
            })),
            source: Some(Box::new(models::PullrequestEndpoint {
                branch: Some(Box::new(models::PullRequestBranch {
                    name: Some("feature/auth".into()),
                    ..Default::default()
                })),
                ..Default::default()
            })),
            destination: Some(Box::new(models::PullrequestEndpoint {
                branch: Some(Box::new(models::PullRequestBranch {
                    name: Some("main".into()),
                    ..Default::default()
                })),
                ..Default::default()
            })),
            draft: Some(false),
            comment_count: Some(3),
            task_count: Some(1),
            created_on: Some("2026-04-15T12:00:00+00:00".into()),
            updated_on: Some("2026-04-16T12:00:00+00:00".into()),
            ..Default::default()
        };

        let row = pull_request_to_row("acme", "api", pull_request).expect("row should exist");

        assert_eq!(row.id, 42);
        assert_eq!(row.state, "OPEN");
        assert_eq!(
            row.html_url.as_deref(),
            Some("https://bitbucket.org/acme/api/pull-requests/42")
        );
        assert_eq!(row.author_display_name.as_deref(), Some("Alice"));
        assert_eq!(row.source_branch.as_deref(), Some("feature/auth"));
        assert_eq!(row.destination_branch.as_deref(), Some("main"));
    }

    #[test]
    fn list_pull_requests_forwards_state_and_follows_pagination() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("listener should bind");
        let address = listener.local_addr().expect("listener address");
        let requests = Arc::new(Mutex::new(Vec::<String>::new()));
        let requests_for_thread = Arc::clone(&requests);

        let server = thread::spawn(move || {
            for _ in 0..2 {
                let (mut stream, _) = listener.accept().expect("request should connect");
                let mut buf = [0_u8; 4096];
                let read = stream.read(&mut buf).expect("request should be readable");
                let request = String::from_utf8_lossy(&buf[..read]).to_string();
                let path = request
                    .lines()
                    .next()
                    .and_then(|line| line.split_whitespace().nth(1))
                    .expect("request path")
                    .to_owned();
                requests_for_thread
                    .lock()
                    .expect("requests lock")
                    .push(path.clone());

                let body = if path.starts_with("/repositories/acme/api/pullrequests?state=MERGED") {
                    format!(
                        "{{\"next\":\"http://{address}/next-page\",\"values\":[{{\"type\":\"pullrequest\",\"id\":1,\"title\":\"First\",\"state\":\"MERGED\"}}]}}"
                    )
                } else {
                    "{\"values\":[{\"type\":\"pullrequest\",\"id\":2,\"title\":\"Second\",\"state\":\"MERGED\"}]}".to_string()
                };

                write!(
                    stream,
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    body.len(),
                    body
                )
                .expect("response should be writable");
            }
        });

        let mut config = Configuration::new();
        config.base_path = format!("http://{address}");
        config.user_agent = Some("bb-test".into());
        config.basic_auth = Some(("alice".into(), Some("token".into())));

        let client = BitbucketClient::from_configuration(config);
        let runtime = Runtime::new().expect("runtime should be created");
        let pull_requests = runtime
            .block_on(client.list_pull_requests_in_repository("acme", "api", Some("MERGED")))
            .expect("request should succeed");

        server.join().expect("server thread should finish");

        assert_eq!(pull_requests.len(), 2);
        assert_eq!(pull_requests[0].id, 1);
        assert_eq!(pull_requests[1].id, 2);

        let requests = requests.lock().expect("requests lock");
        assert_eq!(
            requests[0],
            "/repositories/acme/api/pullrequests?state=MERGED"
        );
        assert_eq!(requests[1], "/next-page");
    }
}
