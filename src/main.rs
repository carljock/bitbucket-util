mod auth;
mod cli;
mod client;
mod error;
mod git;
mod model;
mod output;

use std::process;

use auth::load_credentials;
use cli::{Command, parse_args};
use client::BitbucketClient;
use error::BbcliError;

const BITBUCKET_MACHINE: &str = "api.bitbucket.org";

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("error: {err}");

        if let BbcliError::Api {
            status: Some(401), ..
        } = err
        {
            eprintln!(
                "hint: verify your personal API token, token scopes, and the `machine {BITBUCKET_MACHINE}` entry in ~/.netrc"
            );
        }

        process::exit(1);
    }
}

async fn run() -> Result<(), BbcliError> {
    let parsed = parse_args(std::env::args().skip(1).collect::<Vec<_>>())
        .map_err(|message| BbcliError::Cli { message })?;

    match parsed.command {
        Command::ReposList { workspace, role } => {
            run_repos_list(parsed.json, workspace, role).await
        }
        Command::PullRequestsList { repo, state } => {
            run_pull_requests_list(parsed.json, repo, state).await
        }
    }
}

async fn run_repos_list(
    json: bool,
    workspace: Option<String>,
    role: Option<cli::RepoRole>,
) -> Result<(), BbcliError> {
    let credentials = load_credentials(BITBUCKET_MACHINE)?;
    let client = BitbucketClient::from_credentials(credentials.login, credentials.token);

    let repositories = client
        .list_accessible_repositories(workspace.as_deref(), role.map(|r| r.as_api_value()))
        .await?;

    if json {
        output::write_repositories_json(std::io::stdout(), repositories.as_slice())
            .map_err(|source| BbcliError::with_io("writing CLI output", source))
    } else {
        output::write_repositories(std::io::stdout(), repositories.as_slice())
            .map_err(|source| BbcliError::with_io("writing CLI output", source))
    }
}

async fn run_pull_requests_list(
    json: bool,
    repo: Option<(String, String)>,
    state: Option<cli::PullRequestState>,
) -> Result<(), BbcliError> {
    let credentials = load_credentials(BITBUCKET_MACHINE)?;
    let client = BitbucketClient::from_credentials(credentials.login, credentials.token);
    let (workspace_slug, repo_slug) = resolve_pr_repo_slug(repo)?;

    let pull_requests = client
        .list_pull_requests_in_repository(
            workspace_slug.as_str(),
            repo_slug.as_str(),
            state.map(|value| value.as_api_value()),
        )
        .await?;

    if json {
        output::write_pull_requests_json(std::io::stdout(), pull_requests.as_slice())
            .map_err(|source| BbcliError::with_io("writing CLI output", source))
    } else {
        output::write_pull_requests(std::io::stdout(), pull_requests.as_slice())
            .map_err(|source| BbcliError::with_io("writing CLI output", source))
    }
}

fn resolve_pr_repo_slug(repo: Option<(String, String)>) -> Result<(String, String), BbcliError> {
    match repo {
        Some(repo) => Ok(repo),
        None => git::detect_bitbucket_repo_slug_from_cwd(),
    }
}

#[cfg(test)]
mod tests {
    use super::resolve_pr_repo_slug;

    #[test]
    fn explicit_repo_bypasses_git_detection() {
        let repo = resolve_pr_repo_slug(Some(("acme".into(), "api".into())))
            .expect("explicit repo should succeed");
        assert_eq!(repo, ("acme".into(), "api".into()));
    }
}
