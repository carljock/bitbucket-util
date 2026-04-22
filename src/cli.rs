use clap::{Args, Parser, Subcommand, ValueEnum};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Eq, PartialEq, ValueEnum, Serialize, Deserialize, JsonSchema)]
#[clap(rename_all = "lower")]
#[serde(rename_all = "lowercase")]
pub enum RepoRole {
    Member,
    Contributor,
    Admin,
    Owner,
}

impl RepoRole {
    pub fn as_api_value(self) -> &'static str {
        match self {
            RepoRole::Member => "member",
            RepoRole::Contributor => "contributor",
            RepoRole::Admin => "admin",
            RepoRole::Owner => "owner",
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, ValueEnum, Serialize, Deserialize, JsonSchema)]
#[clap(rename_all = "lower")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PullRequestState {
    Open,
    Merged,
    Declined,
    Superseded,
}

impl PullRequestState {
    pub fn as_api_value(self) -> &'static str {
        match self {
            PullRequestState::Open => "OPEN",
            PullRequestState::Merged => "MERGED",
            PullRequestState::Declined => "DECLINED",
            PullRequestState::Superseded => "SUPERSEDED",
        }
    }
}

#[derive(Debug, Parser, Clone, Eq, PartialEq)]
#[command(name = "bb")]
#[command(version)]
#[command(about = "Bitbucket CLI", long_about = None)]
pub struct ParsedArgs {
    /// Output in JSON format
    #[arg(long, global = true)]
    pub json: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand, Clone, Eq, PartialEq)]
pub enum Command {
    /// Manage repositories
    Repo(RepoArgs),
    /// Manage pull requests
    Pr(PrArgs),
    /// Start an MCP server over stdio
    Mcp,
}

#[derive(Debug, Args, Clone, Eq, PartialEq)]
pub struct RepoArgs {
    #[command(subcommand)]
    pub command: RepoSubcommand,
}

#[derive(Debug, Subcommand, Clone, Eq, PartialEq)]
pub enum RepoSubcommand {
    /// List repositories
    List {
        /// Filter by workspace slug
        #[arg(long)]
        workspace: Option<String>,
        /// Filter by role
        #[arg(long)]
        role: Option<RepoRole>,
    },
}

#[derive(Debug, Args, Clone, Eq, PartialEq)]
pub struct PrArgs {
    #[command(subcommand)]
    pub command: PrSubcommand,
}

#[derive(Debug, Subcommand, Clone, Eq, PartialEq)]
pub enum PrSubcommand {
    /// List pull requests
    List {
        /// Repository in <workspace>/<repo> format
        #[arg(long, value_parser = parse_repo_slug)]
        repo: Option<(String, String)>,
        /// Filter by state
        #[arg(long)]
        state: Option<PullRequestState>,
    },
    /// Get detailed information about a pull request
    Get {
        /// Pull request ID
        id: i32,
        /// Repository in <workspace>/<repo> format
        #[arg(long, value_parser = parse_repo_slug)]
        repo: Option<(String, String)>,
    },
    /// Get the diff of a pull request
    Diff {
        /// Pull request ID
        id: i32,
        /// Repository in <workspace>/<repo> format
        #[arg(long, value_parser = parse_repo_slug)]
        repo: Option<(String, String)>,
    },
    /// List comments on a pull request
    Comments {
        /// Pull request ID
        id: i32,
        /// Repository in <workspace>/<repo> format
        #[arg(long, value_parser = parse_repo_slug)]
        repo: Option<(String, String)>,
    },
    /// Add a comment to a pull request
    Comment {
        /// Pull request ID
        id: i32,
        /// Comment content
        content: String,
        /// Repository in <workspace>/<repo> format
        #[arg(long, value_parser = parse_repo_slug)]
        repo: Option<(String, String)>,
        /// File path for inline comment
        #[arg(long, requires = "line")]
        file: Option<String>,
        /// Line number for inline comment
        #[arg(long, requires = "file")]
        line: Option<i32>,
    },
    /// Approve a pull request
    Approve {
        /// Pull request ID
        id: i32,
        /// Repository in <workspace>/<repo> format
        #[arg(long, value_parser = parse_repo_slug)]
        repo: Option<(String, String)>,
    },
    /// Remove approval from a pull request
    Unapprove {
        /// Pull request ID
        id: i32,
        /// Repository in <workspace>/<repo> format
        #[arg(long, value_parser = parse_repo_slug)]
        repo: Option<(String, String)>,
    },
    /// Decline a pull request
    Decline {
        /// Pull request ID
        id: i32,
        /// Repository in <workspace>/<repo> format
        #[arg(long, value_parser = parse_repo_slug)]
        repo: Option<(String, String)>,
    },
    /// Merge a pull request
    Merge {
        /// Pull request ID
        id: i32,
        /// Repository in <workspace>/<repo> format
        #[arg(long, value_parser = parse_repo_slug)]
        repo: Option<(String, String)>,
        /// Merge commit message
        #[arg(long)]
        message: Option<String>,
        /// Close source branch after merge
        #[arg(long)]
        close_source_branch: bool,
        /// Merge strategy (merge_commit, squash, fast_forward, etc.)
        #[arg(long)]
        strategy: Option<String>,
    },
}

fn parse_repo_slug(value: &str) -> Result<(String, String), String> {
    let Some((workspace_slug, repo_slug)) = value.split_once('/') else {
        return Err(format!(
            "invalid value for --repo `{value}`. Expected <workspace>/<repo>"
        ));
    };

    if workspace_slug.is_empty()
        || repo_slug.is_empty()
        || repo_slug.contains('/')
        || workspace_slug.contains('/')
    {
        return Err(format!(
            "invalid value for --repo `{value}`. Expected <workspace>/<repo>"
        ));
    }

    Ok((workspace_slug.to_owned(), repo_slug.to_owned()))
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use super::{
        Command, ParsedArgs, PrArgs, PrSubcommand, PullRequestState, RepoArgs, RepoRole,
        RepoSubcommand,
    };

    fn parse_args(args: Vec<String>) -> Result<ParsedArgs, clap::Error> {
        let mut full_args = vec!["bb".to_string()];
        full_args.extend(args);
        ParsedArgs::try_parse_from(full_args)
    }

    #[test]
    fn parses_minimal_repo_list() {
        let cmd = parse_args(vec!["repo".into(), "list".into()]).expect("parse should work");

        assert_eq!(
            cmd,
            ParsedArgs {
                json: false,
                command: Command::Repo(RepoArgs {
                    command: RepoSubcommand::List {
                        workspace: None,
                        role: None
                    }
                })
            }
        );
    }

    #[test]
    fn parses_optional_flags() {
        let cmd = parse_args(vec![
            "repo".into(),
            "list".into(),
            "--workspace".into(),
            "acme".into(),
            "--role".into(),
            "contributor".into(),
        ])
        .expect("parse should work");

        assert_eq!(
            cmd,
            ParsedArgs {
                json: false,
                command: Command::Repo(RepoArgs {
                    command: RepoSubcommand::List {
                        workspace: Some("acme".into()),
                        role: Some(RepoRole::Contributor)
                    }
                })
            }
        );
    }

    #[test]
    fn parses_global_json_before_command() {
        let cmd = parse_args(vec!["--json".into(), "repo".into(), "list".into()])
            .expect("parse should work");

        assert_eq!(
            cmd,
            ParsedArgs {
                json: true,
                command: Command::Repo(RepoArgs {
                    command: RepoSubcommand::List {
                        workspace: None,
                        role: None
                    }
                })
            }
        );
    }

    #[test]
    fn parses_global_json_after_command() {
        let cmd = parse_args(vec!["repo".into(), "list".into(), "--json".into()])
            .expect("parse should work");

        assert_eq!(
            cmd,
            ParsedArgs {
                json: true,
                command: Command::Repo(RepoArgs {
                    command: RepoSubcommand::List {
                        workspace: None,
                        role: None
                    }
                })
            }
        );
    }

    #[test]
    fn parses_minimal_pr_list() {
        let cmd = parse_args(vec![
            "pr".into(),
            "list".into(),
            "--repo".into(),
            "acme/api".into(),
        ])
        .expect("parse should work");

        assert_eq!(
            cmd,
            ParsedArgs {
                json: false,
                command: Command::Pr(PrArgs {
                    command: PrSubcommand::List {
                        repo: Some(("acme".into(), "api".into())),
                        state: None,
                    }
                })
            }
        );
    }

    #[test]
    fn parses_pr_list_with_state() {
        let cmd = parse_args(vec![
            "pr".into(),
            "list".into(),
            "--repo".into(),
            "acme/api".into(),
            "--state".into(),
            "merged".into(),
        ])
        .expect("parse should work");

        assert_eq!(
            cmd,
            ParsedArgs {
                json: false,
                command: Command::Pr(PrArgs {
                    command: PrSubcommand::List {
                        repo: Some(("acme".into(), "api".into())),
                        state: Some(PullRequestState::Merged),
                    }
                })
            }
        );
    }

    #[test]
    fn parses_global_json_around_pr_list() {
        let cmd = parse_args(vec![
            "pr".into(),
            "list".into(),
            "--repo".into(),
            "acme/api".into(),
            "--json".into(),
        ])
        .expect("parse should work");

        assert_eq!(
            cmd,
            ParsedArgs {
                json: true,
                command: Command::Pr(PrArgs {
                    command: PrSubcommand::List {
                        repo: Some(("acme".into(), "api".into())),
                        state: None,
                    }
                })
            }
        );
    }

    #[test]
    fn parses_pr_list_without_repo() {
        let cmd = parse_args(vec!["pr".into(), "list".into()]).expect("parse should work");

        assert_eq!(
            cmd,
            ParsedArgs {
                json: false,
                command: Command::Pr(PrArgs {
                    command: PrSubcommand::List {
                        repo: None,
                        state: None,
                    }
                })
            }
        );
    }

    #[test]
    fn parses_pr_list_without_repo_with_state() {
        let cmd = parse_args(vec![
            "pr".into(),
            "list".into(),
            "--state".into(),
            "merged".into(),
        ])
        .expect("parse should work");

        assert_eq!(
            cmd,
            ParsedArgs {
                json: false,
                command: Command::Pr(PrArgs {
                    command: PrSubcommand::List {
                        repo: None,
                        state: Some(PullRequestState::Merged),
                    }
                })
            }
        );
    }

    #[test]
    fn rejects_invalid_repo_shape() {
        let err = parse_args(vec![
            "pr".into(),
            "list".into(),
            "--repo".into(),
            "acme".into(),
        ])
        .expect_err("parse should fail");

        assert!(err.to_string().contains("invalid value for --repo `acme`"));
    }

    #[test]
    fn rejects_invalid_pr_state() {
        let err = parse_args(vec![
            "pr".into(),
            "list".into(),
            "--repo".into(),
            "acme/api".into(),
            "--state".into(),
            "draft".into(),
        ])
        .expect_err("parse should fail");

        assert!(
            err.to_string()
                .contains("invalid value 'draft' for '--state")
        );
    }

    #[test]
    fn parses_mcp_command() {
        let cmd = parse_args(vec!["mcp".into()]).expect("parse should work");

        assert_eq!(
            cmd,
            ParsedArgs {
                json: false,
                command: Command::Mcp,
            }
        );
    }
}
