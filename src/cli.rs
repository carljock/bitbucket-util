const USAGE: &str =
    "Usage:\n  bbcli repos list [--workspace <slug>] [--role <member|contributor|admin|owner>]";

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

pub fn parse_args(args: impl IntoIterator<Item = String>) -> Result<Command, String> {
    let args: Vec<String> = args.into_iter().collect();

    if args.len() < 2 {
        return Err(format!("missing command\n\n{USAGE}"));
    }

    if args[0] != "repos" || args[1] != "list" {
        return Err(format!("unsupported command\n\n{USAGE}"));
    }

    let mut workspace = None;
    let mut role = None;

    let mut i = 2;
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
            flag => {
                return Err(format!("unknown option `{flag}`\n\n{USAGE}"));
            }
        }
    }

    Ok(Command::ReposList { workspace, role })
}

#[cfg(test)]
mod tests {
    use super::{Command, RepoRole, parse_args};

    #[test]
    fn parses_minimal_repos_list() {
        let cmd = parse_args(vec!["repos".into(), "list".into()]).expect("parse should work");

        assert_eq!(
            cmd,
            Command::ReposList {
                workspace: None,
                role: None
            }
        );
    }

    #[test]
    fn parses_optional_flags() {
        let cmd = parse_args(vec![
            "repos".into(),
            "list".into(),
            "--workspace".into(),
            "acme".into(),
            "--role".into(),
            "contributor".into(),
        ])
        .expect("parse should work");

        assert_eq!(
            cmd,
            Command::ReposList {
                workspace: Some("acme".into()),
                role: Some(RepoRole::Contributor)
            }
        );
    }
}
