mod auth;
mod cli;
mod client;
mod error;
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
    let command = parse_args(std::env::args().skip(1).collect::<Vec<_>>())
        .map_err(|message| BbcliError::Cli { message })?;

    match command {
        Command::ReposList { workspace, role } => run_repos_list(workspace, role).await,
    }
}

async fn run_repos_list(
    workspace: Option<String>,
    role: Option<cli::RepoRole>,
) -> Result<(), BbcliError> {
    let credentials = load_credentials(BITBUCKET_MACHINE)?;
    let client = BitbucketClient::from_credentials(credentials.login, credentials.token);

    let repositories = client
        .list_accessible_repositories(workspace.as_deref(), role.map(|r| r.as_api_value()))
        .await?;

    output::write_repositories(std::io::stdout(), repositories.as_slice())
        .map_err(|source| BbcliError::with_io("writing CLI output", source))
}
