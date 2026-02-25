use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::BbcliError;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Credentials {
    pub login: String,
    pub token: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct MachineEntry {
    machine: String,
    login: Option<String>,
    password: Option<String>,
}

pub fn load_credentials(machine: &str) -> Result<Credentials, BbcliError> {
    let home_dir = env::var_os("HOME").ok_or(BbcliError::HomeDirUnavailable)?;
    let netrc_path = PathBuf::from(home_dir).join(".netrc");
    load_credentials_from_path(&netrc_path, machine)
}

pub fn load_credentials_from_path(path: &Path, machine: &str) -> Result<Credentials, BbcliError> {
    let content = fs::read_to_string(path).map_err(|source| BbcliError::NetrcRead {
        path: path.to_path_buf(),
        source,
    })?;

    let entries = parse_machine_entries(&content)?;

    let Some(entry) = entries.into_iter().find(|entry| entry.machine == machine) else {
        return Err(BbcliError::NetrcMachineNotFound {
            path: path.to_path_buf(),
            machine: machine.to_owned(),
        });
    };

    let Some(login) = entry.login else {
        return Err(BbcliError::NetrcLoginMissing {
            machine: machine.to_owned(),
        });
    };

    let Some(password) = entry.password else {
        return Err(BbcliError::NetrcPasswordMissing {
            machine: machine.to_owned(),
        });
    };

    Ok(Credentials {
        login,
        token: password,
    })
}

fn parse_machine_entries(content: &str) -> Result<Vec<MachineEntry>, BbcliError> {
    let tokens = tokenize(content);

    if tokens.is_empty() {
        return Err(BbcliError::NetrcParse {
            message: "file is empty".to_string(),
        });
    }

    let mut entries = Vec::new();
    let mut current: Option<MachineEntry> = None;
    let mut i = 0;

    while i < tokens.len() {
        match tokens[i].as_str() {
            "machine" => {
                if let Some(entry) = current.take() {
                    entries.push(entry);
                }

                let Some(machine) = tokens.get(i + 1) else {
                    return Err(BbcliError::NetrcParse {
                        message: "`machine` without value".to_string(),
                    });
                };

                current = Some(MachineEntry {
                    machine: machine.clone(),
                    login: None,
                    password: None,
                });
                i += 2;
            }
            "login" => {
                let Some(value) = tokens.get(i + 1) else {
                    return Err(BbcliError::NetrcParse {
                        message: "`login` without value".to_string(),
                    });
                };

                if let Some(entry) = current.as_mut() {
                    entry.login = Some(value.clone());
                }
                i += 2;
            }
            "password" => {
                let Some(value) = tokens.get(i + 1) else {
                    return Err(BbcliError::NetrcParse {
                        message: "`password` without value".to_string(),
                    });
                };

                if let Some(entry) = current.as_mut() {
                    entry.password = Some(value.clone());
                }
                i += 2;
            }
            _ => {
                i += 1;
            }
        }
    }

    if let Some(entry) = current {
        entries.push(entry);
    }

    Ok(entries)
}

fn tokenize(content: &str) -> Vec<String> {
    let mut tokens = Vec::new();

    for line in content.lines() {
        let line_without_comment = line.split('#').next().unwrap_or_default();
        tokens.extend(line_without_comment.split_whitespace().map(str::to_owned));
    }

    tokens
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use super::load_credentials_from_path;

    fn write_tmp_netrc(contents: &str, suffix: &str) -> PathBuf {
        let path =
            std::env::temp_dir().join(format!("bbcli-netrc-{suffix}-{}.txt", std::process::id()));
        fs::write(&path, contents).expect("netrc write should succeed");
        path
    }

    #[test]
    fn loads_credentials_for_machine() {
        let path = write_tmp_netrc(
            "machine api.bitbucket.org\n  login alice\n  password token123\n",
            "ok",
        );

        let creds =
            load_credentials_from_path(&path, "api.bitbucket.org").expect("credentials expected");

        assert_eq!(creds.login, "alice");
        assert_eq!(creds.token, "token123");

        fs::remove_file(path).expect("cleanup should succeed");
    }

    #[test]
    fn fails_when_machine_missing() {
        let path = write_tmp_netrc(
            "machine example.com\n login x\n password y\n",
            "missing-machine",
        );

        let err = load_credentials_from_path(&path, "api.bitbucket.org")
            .expect_err("missing machine should fail");

        assert!(
            err.to_string()
                .contains("credentials for machine `api.bitbucket.org`")
        );

        fs::remove_file(path).expect("cleanup should succeed");
    }

    #[test]
    fn fails_when_login_missing() {
        let path = write_tmp_netrc(
            "machine api.bitbucket.org\n  password token\n",
            "missing-login",
        );

        let err = load_credentials_from_path(&path, "api.bitbucket.org")
            .expect_err("missing login should fail");

        assert!(err.to_string().contains("missing `login`"));

        fs::remove_file(path).expect("cleanup should succeed");
    }

    #[test]
    fn fails_when_password_missing() {
        let path = write_tmp_netrc(
            "machine api.bitbucket.org\n  login alice\n",
            "missing-password",
        );

        let err = load_credentials_from_path(&path, "api.bitbucket.org")
            .expect_err("missing password should fail");

        assert!(err.to_string().contains("missing `password`"));

        fs::remove_file(path).expect("cleanup should succeed");
    }
}
