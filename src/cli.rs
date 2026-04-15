const USAGE: &str =
    "Usage:\n  bb [--json] repo list [--workspace <slug>] [--role <member|contributor|admin|owner>]";

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

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Command {
    ReposList {
        workspace: Option<String>,
        role: Option<RepoRole>,
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

    if positionals[0] != "repo" || positionals[1] != "list" {
        return Err(format!("unsupported command\n\n{USAGE}"));
    }

    let mut workspace = None;
    let mut role = None;

    let mut i = 2;
    while i < positionals.len() {
        match positionals[i].as_str() {
            "--workspace" => {
                let Some(value) = positionals.get(i + 1) else {
                    return Err(format!("missing value for --workspace\n\n{USAGE}"));
                };

                workspace = Some(value.clone());
                i += 2;
            }
            "--role" => {
                let Some(value) = positionals.get(i + 1) else {
                    return Err(format!("missing value for --role\n\n{USAGE}"));
                };

                role = Some(RepoRole::parse(value)?);
                i += 2;
            }
            flag => {
                return Err(format!("unknown option `{flag}`\n\n{USAGE}"));
            }
        }
    }

    Ok(ParsedArgs {
        json,
        command: Command::ReposList { workspace, role },
    })
}

#[cfg(test)]
mod tests {
    use super::{Command, ParsedArgs, RepoRole, parse_args};

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
}
