const USAGE: &str = concat!(
    "Usage:\n",
    "  bb [--json] repo list [--workspace <slug>] ",
    "[--role <member|contributor|admin|owner>]\n",
    "  bb [--json] pr list --repo <workspace>/<repo> ",
    "[--state <open|merged|declined|superseded>]"
);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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

    fn parse(value: &str) -> Result<Self, String> {
        match value {
            "member" => Ok(Self::Member),
            "contributor" => Ok(Self::Contributor),
            "admin" => Ok(Self::Admin),
            "owner" => Ok(Self::Owner),
            _ => Err(format!(
                "invalid value for --role `{value}`. Expected one of: member, contributor, admin, owner"
            )),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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

    fn parse(value: &str) -> Result<Self, String> {
        match value {
            "open" => Ok(Self::Open),
            "merged" => Ok(Self::Merged),
            "declined" => Ok(Self::Declined),
            "superseded" => Ok(Self::Superseded),
            _ => Err(format!(
                "invalid value for --state `{value}`. Expected one of: open, merged, declined, superseded"
            )),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Command {
    ReposList {
        workspace: Option<String>,
        role: Option<RepoRole>,
    },
    PullRequestsList {
        workspace_slug: String,
        repo_slug: String,
        state: Option<PullRequestState>,
    },
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ParsedArgs {
    pub json: bool,
    pub command: Command,
}

pub fn parse_args(args: impl IntoIterator<Item = String>) -> Result<ParsedArgs, String> {
    let args: Vec<String> = args.into_iter().collect();

    if args.is_empty() {
        return Err(format!("missing command\n\n{USAGE}"));
    }

    let mut json = false;
    let mut positionals = Vec::new();

    for arg in args {
        match arg.as_str() {
            "--json" => json = true,
            _ => positionals.push(arg),
        }
    }

    if positionals.len() < 2 {
        return Err(format!("missing command\n\n{USAGE}"));
    }

    let command = match (positionals[0].as_str(), positionals[1].as_str()) {
        ("repo", "list") => parse_repo_list(&positionals[2..])?,
        ("pr", "list") => parse_pr_list(&positionals[2..])?,
        _ => return Err(format!("unsupported command\n\n{USAGE}")),
    };

    Ok(ParsedArgs { json, command })
}

fn parse_repo_list(args: &[String]) -> Result<Command, String> {
    let mut workspace = None;
    let mut role = None;
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "--workspace" => {
                let Some(value) = args.get(i + 1) else {
                    return Err(format!("missing value for --workspace\n\n{USAGE}"));
                };

                workspace = Some(value.clone());
                i += 2;
            }
            "--role" => {
                let Some(value) = args.get(i + 1) else {
                    return Err(format!("missing value for --role\n\n{USAGE}"));
                };

                role = Some(RepoRole::parse(value)?);
                i += 2;
            }
            flag => return Err(format!("unknown option `{flag}`\n\n{USAGE}")),
        }
    }

    Ok(Command::ReposList { workspace, role })
}

fn parse_pr_list(args: &[String]) -> Result<Command, String> {
    let mut repo = None;
    let mut state = None;
    let mut i = 0;

    while i < args.len() {
        match args[i].as_str() {
            "--repo" => {
                let Some(value) = args.get(i + 1) else {
                    return Err(format!("missing value for --repo\n\n{USAGE}"));
                };

                repo = Some(parse_repo_slug(value)?);
                i += 2;
            }
            "--state" => {
                let Some(value) = args.get(i + 1) else {
                    return Err(format!("missing value for --state\n\n{USAGE}"));
                };

                state = Some(PullRequestState::parse(value)?);
                i += 2;
            }
            flag => return Err(format!("unknown option `{flag}`\n\n{USAGE}")),
        }
    }

    let Some((workspace_slug, repo_slug)) = repo else {
        return Err(format!("missing required option --repo\n\n{USAGE}"));
    };

    Ok(Command::PullRequestsList {
        workspace_slug,
        repo_slug,
        state,
    })
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
    use super::{Command, ParsedArgs, PullRequestState, RepoRole, parse_args};

    #[test]
    fn parses_minimal_repo_list() {
        let cmd = parse_args(vec!["repo".into(), "list".into()]).expect("parse should work");

        assert_eq!(
            cmd,
            ParsedArgs {
                json: false,
                command: Command::ReposList {
                    workspace: None,
                    role: None
                }
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
                command: Command::ReposList {
                    workspace: Some("acme".into()),
                    role: Some(RepoRole::Contributor)
                }
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
                command: Command::ReposList {
                    workspace: None,
                    role: None
                }
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
                command: Command::ReposList {
                    workspace: None,
                    role: None
                }
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
                command: Command::PullRequestsList {
                    workspace_slug: "acme".into(),
                    repo_slug: "api".into(),
                    state: None,
                }
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
                command: Command::PullRequestsList {
                    workspace_slug: "acme".into(),
                    repo_slug: "api".into(),
                    state: Some(PullRequestState::Merged),
                }
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
                command: Command::PullRequestsList {
                    workspace_slug: "acme".into(),
                    repo_slug: "api".into(),
                    state: None,
                }
            }
        );
    }

    #[test]
    fn rejects_pr_list_without_repo() {
        let err = parse_args(vec!["pr".into(), "list".into()]).expect_err("parse should fail");
        assert!(err.contains("missing required option --repo"));
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

        assert!(err.contains("invalid value for --repo `acme`"));
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

        assert!(err.contains("invalid value for --state `draft`"));
    }
}
