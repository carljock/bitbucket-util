mod auth;
mod cli;
mod client;
mod error;
mod git;
mod mcp;
mod model;
mod output;

use std::process;

use auth::load_credentials;
use clap::Parser;
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
    let parsed = cli::ParsedArgs::parse();

    match parsed.command {
        cli::Command::Repo(repo_args) => match repo_args.command {
            cli::RepoSubcommand::List { workspace, role } => {
                run_repos_list(parsed.json, workspace, role).await
            }
        },
        cli::Command::Pr(pr_args) => match pr_args.command {
            cli::PrSubcommand::List { repo, state } => {
                run_pull_requests_list(parsed.json, repo, state).await
            }
            cli::PrSubcommand::Get { id, repo } => {
                run_pull_request_get(parsed.json, id, repo).await
            }
            cli::PrSubcommand::Diff { id, repo } => {
                run_pull_request_diff(parsed.json, id, repo).await
            }
            cli::PrSubcommand::Comments { id, repo } => {
                run_pull_request_comments(parsed.json, id, repo).await
            }
            cli::PrSubcommand::Comment {
                id,
                content,
                repo,
                file,
                line,
            } => run_pull_request_comment(parsed.json, id, content, repo, file, line).await,
            cli::PrSubcommand::Approve { id, repo } => {
                run_pull_request_approve(parsed.json, id, repo).await
            }
            cli::PrSubcommand::Unapprove { id, repo } => {
                run_pull_request_unapprove(parsed.json, id, repo).await
            }
            cli::PrSubcommand::Decline { id, repo } => {
                run_pull_request_decline(parsed.json, id, repo).await
            }
            cli::PrSubcommand::Merge {
                id,
                repo,
                message,
                close_source_branch,
                strategy,
            } => {
                run_pull_request_merge(
                    parsed.json,
                    id,
                    repo,
                    message,
                    close_source_branch,
                    strategy,
                )
                .await
            }
        },
        cli::Command::Mcp => run_mcp(parsed.json).await,
    }
}

async fn run_mcp(json: bool) -> Result<(), BbcliError> {
    if json {
        return Err(BbcliError::Usage {
            message: "`--json` is not supported with `bb mcp`".to_owned(),
        });
    }

    mcp::run_server().await
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

async fn run_pull_request_get(
    json: bool,
    id: i32,
    repo: Option<(String, String)>,
) -> Result<(), BbcliError> {
    let credentials = load_credentials(BITBUCKET_MACHINE)?;
    let client = BitbucketClient::from_credentials(credentials.login, credentials.token);
    let (workspace_slug, repo_slug) = resolve_pr_repo_slug(repo)?;

    let pr = client
        .get_pull_request(&workspace_slug, &repo_slug, id)
        .await?;

    if json {
        output::write_pull_request_json(std::io::stdout(), &pr)
            .map_err(|source| BbcliError::with_io("writing CLI output", source))
    } else {
        output::write_pull_request(std::io::stdout(), &pr)
            .map_err(|source| BbcliError::with_io("writing CLI output", source))
    }
}

async fn run_pull_request_diff(
    json: bool,
    id: i32,
    repo: Option<(String, String)>,
) -> Result<(), BbcliError> {
    if json {
        return Err(BbcliError::Usage {
            message: "`--json` is not supported with `bb pr diff`".to_owned(),
        });
    }

    let credentials = load_credentials(BITBUCKET_MACHINE)?;
    let client = BitbucketClient::from_credentials(credentials.login, credentials.token);
    let (workspace_slug, repo_slug) = resolve_pr_repo_slug(repo)?;

    let diff = client
        .get_pull_request_diff(&workspace_slug, &repo_slug, id)
        .await?;

    print!("{}", diff);
    Ok(())
}

async fn run_pull_request_comments(
    json: bool,
    id: i32,
    repo: Option<(String, String)>,
) -> Result<(), BbcliError> {
    let credentials = load_credentials(BITBUCKET_MACHINE)?;
    let client = BitbucketClient::from_credentials(credentials.login, credentials.token);
    let (workspace_slug, repo_slug) = resolve_pr_repo_slug(repo)?;

    let comments = client
        .list_pull_request_comments(&workspace_slug, &repo_slug, id)
        .await?;

    if json {
        output::write_pull_request_comments_json(std::io::stdout(), comments.as_slice())
            .map_err(|source| BbcliError::with_io("writing CLI output", source))
    } else {
        output::write_pull_request_comments(std::io::stdout(), comments.as_slice())
            .map_err(|source| BbcliError::with_io("writing CLI output", source))
    }
}

async fn run_pull_request_comment(
    json: bool,
    id: i32,
    content: String,
    repo: Option<(String, String)>,
    file: Option<String>,
    line: Option<i32>,
) -> Result<(), BbcliError> {
    let credentials = load_credentials(BITBUCKET_MACHINE)?;
    let client = BitbucketClient::from_credentials(credentials.login, credentials.token);
    let (workspace_slug, repo_slug) = resolve_pr_repo_slug(repo)?;

    let inline = match (file, line) {
        (Some(path), Some(to)) => Some(model::PullRequestInlineComment {
            path,
            from: None,
            to: Some(to),
        }),
        _ => None,
    };

    let comment = client
        .create_pull_request_comment(&workspace_slug, &repo_slug, id, &content, inline)
        .await?;

    if json {
        output::write_pull_request_comment_json(std::io::stdout(), &comment)
            .map_err(|source| BbcliError::with_io("writing CLI output", source))
    } else {
        output::write_pull_request_comment(std::io::stdout(), &comment)
            .map_err(|source| BbcliError::with_io("writing CLI output", source))
    }
}

async fn run_pull_request_approve(
    json: bool,
    id: i32,
    repo: Option<(String, String)>,
) -> Result<(), BbcliError> {
    if json {
        return Err(BbcliError::Usage {
            message: "`--json` is not supported with `bb pr approve`".to_owned(),
        });
    }

    let credentials = load_credentials(BITBUCKET_MACHINE)?;
    let client = BitbucketClient::from_credentials(credentials.login, credentials.token);
    let (workspace_slug, repo_slug) = resolve_pr_repo_slug(repo)?;

    client
        .approve_pull_request(&workspace_slug, &repo_slug, id)
        .await?;

    println!("Pull request #{} approved.", id);
    Ok(())
}

async fn run_pull_request_unapprove(
    json: bool,
    id: i32,
    repo: Option<(String, String)>,
) -> Result<(), BbcliError> {
    if json {
        return Err(BbcliError::Usage {
            message: "`--json` is not supported with `bb pr unapprove`".to_owned(),
        });
    }

    let credentials = load_credentials(BITBUCKET_MACHINE)?;
    let client = BitbucketClient::from_credentials(credentials.login, credentials.token);
    let (workspace_slug, repo_slug) = resolve_pr_repo_slug(repo)?;

    client
        .unapprove_pull_request(&workspace_slug, &repo_slug, id)
        .await?;

    println!("Pull request #{} unapproved.", id);
    Ok(())
}

async fn run_pull_request_decline(
    json: bool,
    id: i32,
    repo: Option<(String, String)>,
) -> Result<(), BbcliError> {
    if json {
        return Err(BbcliError::Usage {
            message: "`--json` is not supported with `bb pr decline`".to_owned(),
        });
    }

    let credentials = load_credentials(BITBUCKET_MACHINE)?;
    let client = BitbucketClient::from_credentials(credentials.login, credentials.token);
    let (workspace_slug, repo_slug) = resolve_pr_repo_slug(repo)?;

    client
        .decline_pull_request(&workspace_slug, &repo_slug, id)
        .await?;

    println!("Pull request #{} declined.", id);
    Ok(())
}

async fn run_pull_request_merge(
    json: bool,
    id: i32,
    repo: Option<(String, String)>,
    message: Option<String>,
    close_source_branch: bool,
    strategy: Option<String>,
) -> Result<(), BbcliError> {
    if json {
        return Err(BbcliError::Usage {
            message: "`--json` is not supported with `bb pr merge`".to_owned(),
        });
    }

    let credentials = load_credentials(BITBUCKET_MACHINE)?;
    let client = BitbucketClient::from_credentials(credentials.login, credentials.token);
    let (workspace_slug, repo_slug) = resolve_pr_repo_slug(repo)?;

    client
        .merge_pull_request(
            &workspace_slug,
            &repo_slug,
            id,
            message,
            Some(close_source_branch),
            strategy.as_deref(),
        )
        .await?;

    println!("Pull request #{} merged.", id);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{resolve_pr_repo_slug, run_mcp};

    #[test]
    fn explicit_repo_bypasses_git_detection() {
        let repo = resolve_pr_repo_slug(Some(("acme".into(), "api".into())))
            .expect("explicit repo should succeed");
        assert_eq!(repo, ("acme".into(), "api".into()));
    }

    #[tokio::test]
    async fn mcp_rejects_json_output_flag() {
        let err = run_mcp(true).await.expect_err("json should be rejected");
        assert_eq!(err.to_string(), "`--json` is not supported with `bb mcp`");
    }
}
