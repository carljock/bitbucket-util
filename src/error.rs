use std::error::Error as StdError;
use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub enum BbcliError {
    Usage {
        message: String,
    },
    HomeDirUnavailable,
    CurrentDirRead {
        source: std::io::Error,
    },
    NetrcRead {
        path: PathBuf,
        source: std::io::Error,
    },
    NetrcParse {
        message: String,
    },
    NetrcMachineNotFound {
        path: PathBuf,
        machine: String,
    },
    NetrcLoginMissing {
        machine: String,
    },
    NetrcPasswordMissing {
        machine: String,
    },
    Api {
        context: &'static str,
        status: Option<u16>,
        message: String,
    },
    Network {
        context: &'static str,
        message: String,
    },
    Parse {
        context: &'static str,
        message: String,
    },
    Io {
        context: &'static str,
        source: std::io::Error,
    },
    GitRepoNotFound,
    GitWorktreeUnsupported {
        path: PathBuf,
    },
    GitConfigRead {
        path: PathBuf,
        source: std::io::Error,
    },
    BitbucketRemoteNotFound,
    BitbucketRemoteAmbiguous {
        remotes: Vec<String>,
    },
    BitbucketRemoteMalformed {
        remote: String,
        url: String,
    },
    Mcp {
        context: &'static str,
        message: String,
    },
}

impl fmt::Display for BbcliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BbcliError::Usage { message } => write!(f, "{message}"),
            BbcliError::HomeDirUnavailable => {
                write!(f, "unable to determine home directory for .netrc lookup")
            }
            BbcliError::CurrentDirRead { source } => {
                write!(f, "failed to determine current directory: {source}")
            }
            BbcliError::NetrcRead { path, source } => {
                write!(f, "failed to read {}: {}", path.display(), source)
            }
            BbcliError::NetrcParse { message } => write!(f, "failed to parse .netrc: {message}"),
            BbcliError::NetrcMachineNotFound { path, machine } => write!(
                f,
                "credentials for machine `{machine}` were not found in {}",
                path.display()
            ),
            BbcliError::NetrcLoginMissing { machine } => {
                write!(f, "machine `{machine}` entry is missing `login`")
            }
            BbcliError::NetrcPasswordMissing { machine } => {
                write!(f, "machine `{machine}` entry is missing `password`")
            }
            BbcliError::Api {
                context,
                status,
                message,
            } => {
                if let Some(status) = status {
                    write!(
                        f,
                        "Bitbucket API error during {context} (HTTP {status}): {message}"
                    )
                } else {
                    write!(f, "Bitbucket API error during {context}: {message}")
                }
            }
            BbcliError::Network { context, message } => {
                write!(f, "network error during {context}: {message}")
            }
            BbcliError::Parse { context, message } => {
                write!(f, "parse error during {context}: {message}")
            }
            BbcliError::Io { context, source } => write!(f, "I/O error during {context}: {source}"),
            BbcliError::GitRepoNotFound => write!(
                f,
                "not inside a git repository; pass --repo <workspace>/<repo>"
            ),
            BbcliError::GitWorktreeUnsupported { path } => write!(
                f,
                "{} is a git file; worktrees are not supported, pass --repo <workspace>/<repo>",
                path.display()
            ),
            BbcliError::GitConfigRead { path, source } => {
                write!(f, "failed to read {}: {}", path.display(), source)
            }
            BbcliError::BitbucketRemoteNotFound => write!(
                f,
                "no Bitbucket remote found in .git/config; pass --repo <workspace>/<repo>"
            ),
            BbcliError::BitbucketRemoteAmbiguous { remotes } => write!(
                f,
                "multiple Bitbucket remotes found ({}); pass --repo <workspace>/<repo>",
                remotes.join(", ")
            ),
            BbcliError::BitbucketRemoteMalformed { remote, url } => write!(
                f,
                "malformed Bitbucket remote `{remote}` with URL `{url}`; pass --repo <workspace>/<repo>"
            ),
            BbcliError::Mcp { context, message } => {
                write!(f, "MCP error during {context}: {message}")
            }
        }
    }
}

impl StdError for BbcliError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            BbcliError::CurrentDirRead { source } => Some(source),
            BbcliError::NetrcRead { source, .. } => Some(source),
            BbcliError::GitConfigRead { source, .. } => Some(source),
            BbcliError::Io { source, .. } => Some(source),
            _ => None,
        }
    }
}

impl BbcliError {
    pub fn with_io(context: &'static str, source: std::io::Error) -> Self {
        Self::Io { context, source }
    }
}
