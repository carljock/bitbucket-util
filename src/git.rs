use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::BbcliError;

pub fn detect_bitbucket_repo_slug_from_cwd() -> Result<(String, String), BbcliError> {
    let cwd = env::current_dir().map_err(|source| BbcliError::CurrentDirRead { source })?;
    detect_bitbucket_repo_slug_from_path(cwd.as_path())
}

pub fn detect_bitbucket_repo_slug_from_path(start: &Path) -> Result<(String, String), BbcliError> {
    let git_dir = find_git_dir(start)?;
    let config_path = git_dir.join("config");
    let config = fs::read_to_string(&config_path).map_err(|source| BbcliError::GitConfigRead {
        path: config_path,
        source,
    })?;

    let remotes = parse_git_config_remotes(config.as_str());
    let mut bitbucket_remotes = Vec::new();

    for (name, url) in remotes {
        match parse_bitbucket_slug_from_remote_url(url.as_str()) {
            Some(Ok(slug)) => bitbucket_remotes.push((name, slug)),
            Some(Err(_)) => {
                return Err(BbcliError::BitbucketRemoteMalformed { remote: name, url });
            }
            None => {}
        }
    }

    match bitbucket_remotes.len() {
        0 => Err(BbcliError::BitbucketRemoteNotFound),
        1 => Ok(bitbucket_remotes.pop().expect("single element").1),
        _ => Err(BbcliError::BitbucketRemoteAmbiguous {
            remotes: bitbucket_remotes
                .into_iter()
                .map(|(name, _)| name)
                .collect(),
        }),
    }
}

fn find_git_dir(start: &Path) -> Result<PathBuf, BbcliError> {
    let mut current = Some(start);

    while let Some(path) = current {
        let git_path = path.join(".git");

        if git_path.is_dir() {
            return Ok(git_path);
        }

        if git_path.is_file() {
            return resolve_git_file(git_path.as_path());
        }

        if git_path.exists() {
            return Err(BbcliError::GitWorktreeUnsupported { path: git_path });
        }

        current = path.parent();
    }

    Err(BbcliError::GitRepoNotFound)
}

fn resolve_git_file(git_path: &Path) -> Result<PathBuf, BbcliError> {
    let content = fs::read_to_string(git_path).map_err(|source| BbcliError::GitConfigRead {
        path: git_path.to_path_buf(),
        source,
    })?;

    let Some(raw_gitdir) = content
        .lines()
        .next()
        .and_then(|line| line.trim().strip_prefix("gitdir:"))
        .map(str::trim)
    else {
        return Err(BbcliError::GitWorktreeUnsupported {
            path: git_path.to_path_buf(),
        });
    };

    if raw_gitdir.is_empty() {
        return Err(BbcliError::GitWorktreeUnsupported {
            path: git_path.to_path_buf(),
        });
    }

    Ok(if Path::new(raw_gitdir).is_absolute() {
        PathBuf::from(raw_gitdir)
    } else {
        git_path
            .parent()
            .unwrap_or_else(|| Path::new("/"))
            .join(raw_gitdir)
    })
}

fn parse_git_config_remotes(config: &str) -> Vec<(String, String)> {
    let mut remotes = Vec::new();
    let mut current_remote = None;

    for line in config.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with('#') || trimmed.starts_with(';') {
            continue;
        }

        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            current_remote = parse_remote_section(trimmed);
            continue;
        }

        let Some(remote_name) = current_remote.as_deref() else {
            continue;
        };

        let Some((key, value)) = trimmed.split_once('=') else {
            continue;
        };

        if key.trim() == "url" {
            remotes.push((remote_name.to_owned(), value.trim().to_owned()));
        }
    }

    remotes
}

fn parse_remote_section(section: &str) -> Option<String> {
    let inner = section.strip_prefix('[')?.strip_suffix(']')?.trim();
    let name = inner.strip_prefix("remote")?.trim();

    if let Some(quoted) = name.strip_prefix('"').and_then(|n| n.strip_suffix('"')) {
        if quoted.is_empty() {
            None
        } else {
            Some(quoted.to_owned())
        }
    } else {
        // Handle unquoted names like [remote origin]
        if name.is_empty() {
            None
        } else {
            Some(name.to_owned())
        }
    }
}

fn parse_bitbucket_slug_from_remote_url(url: &str) -> Option<Result<(String, String), ()>> {
    if let Some(path) = url.strip_prefix("git@bitbucket.org:") {
        return Some(parse_bitbucket_slug_path(path));
    }

    if let Some(rest) = url.strip_prefix("ssh://") {
        // Handle optional username: ssh://user@bitbucket.org
        let rest = if let Some((_user, host_path)) = rest.split_once('@') {
            host_path
        } else {
            rest
        };

        // Handle bitbucket.org followed by / or :port/
        if let Some(path) = rest.strip_prefix("bitbucket.org/") {
            return Some(parse_bitbucket_slug_path(path));
        }

        if let Some(rest) = rest.strip_prefix("bitbucket.org:") {
            if let Some((_port, path)) = rest.split_once('/') {
                return Some(parse_bitbucket_slug_path(path));
            }
        }

        return None;
    }

    if let Some(rest) = url.strip_prefix("https://") {
        // Handle optional username: https://user@bitbucket.org/
        let rest = if let Some((_user, host_path)) = rest.split_once('@') {
            host_path
        } else {
            rest
        };

        if let Some(path) = rest.strip_prefix("bitbucket.org/") {
            return Some(parse_bitbucket_slug_path(path));
        }
    }

    None
}

fn parse_bitbucket_slug_path(path: &str) -> Result<(String, String), ()> {
    let path = path.trim_matches('/');
    let mut parts = path.split('/');
    let workspace = parts.next().ok_or(())?;
    let repo = parts.next().ok_or(())?;

    if parts.next().is_some() || workspace.is_empty() || repo.is_empty() {
        return Err(());
    }

    let repo = repo.strip_suffix(".git").unwrap_or(repo);
    if repo.is_empty() {
        return Err(());
    }

    Ok((workspace.to_owned(), repo.to_owned()))
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::{Mutex, OnceLock};

    use super::{
        detect_bitbucket_repo_slug_from_path, parse_bitbucket_slug_from_remote_url,
        parse_git_config_remotes,
    };

    fn cwd_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    fn write_repo(pathsuffix: &str, config: Option<&str>) -> PathBuf {
        let root = std::env::temp_dir().join(format!(
            "bb-git-{pathsuffix}-{}-{}",
            std::process::id(),
            unique_id()
        ));
        let git_dir = root.join(".git");
        fs::create_dir_all(&git_dir).expect("git dir should be created");

        if let Some(config) = config {
            fs::write(git_dir.join("config"), config).expect("config should be written");
        }

        root
    }

    fn unique_id() -> u128 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time should work")
            .as_nanos()
    }

    fn write_worktree_like_repo(pathsuffix: &str) -> PathBuf {
        let root = std::env::temp_dir().join(format!(
            "bb-git-worktree-{pathsuffix}-{}-{}",
            std::process::id(),
            unique_id()
        ));
        fs::create_dir_all(&root).expect("root should be created");
        let git_dir = root.join(".git-worktree");
        fs::create_dir_all(&git_dir).expect("git dir should be created");
        fs::write(
            root.join(".git"),
            format!("gitdir: {}\n", git_dir.display()),
        )
        .expect("git file should be written");
        fs::write(
            git_dir.join("config"),
            "[remote \"origin\"]\n    url = git@bitbucket.org:acme/worktree.git\n",
        )
        .expect("config should be written");
        root
    }

    fn nested_path(root: &Path) -> PathBuf {
        let nested = root.join("a").join("b");
        fs::create_dir_all(&nested).expect("nested path should be created");
        nested
    }

    #[test]
    fn parses_git_config_remotes_in_order() {
        let config = r#"
[core]
    bare = false
[remote "origin"]
    url = git@bitbucket.org:acme/api.git
[remote github]
    url = git@github.com:acme/api.git
"#;

        let remotes = parse_git_config_remotes(config);
        assert_eq!(
            remotes,
            vec![
                ("origin".into(), "git@bitbucket.org:acme/api.git".into()),
                ("github".into(), "git@github.com:acme/api.git".into())
            ]
        );
    }

    #[test]
    fn parses_bitbucket_remote_urls() {
        assert_eq!(
            parse_bitbucket_slug_from_remote_url("git@bitbucket.org:acme/api.git"),
            Some(Ok(("acme".into(), "api".into())))
        );
        assert_eq!(
            parse_bitbucket_slug_from_remote_url("ssh://git@bitbucket.org/acme/api"),
            Some(Ok(("acme".into(), "api".into())))
        );
        assert_eq!(
            parse_bitbucket_slug_from_remote_url("ssh://git@bitbucket.org:22/acme/api.git"),
            Some(Ok(("acme".into(), "api".into())))
        );
        assert_eq!(
            parse_bitbucket_slug_from_remote_url("https://bitbucket.org/acme/api.git"),
            Some(Ok(("acme".into(), "api".into())))
        );
        assert_eq!(
            parse_bitbucket_slug_from_remote_url("https://user@bitbucket.org/acme/api.git"),
            Some(Ok(("acme".into(), "api".into())))
        );
        assert_eq!(
            parse_bitbucket_slug_from_remote_url(
                "https://x-token-auth:token@bitbucket.org/acme/api.git"
            ),
            Some(Ok(("acme".into(), "api".into())))
        );
    }

    #[test]
    fn rejects_malformed_bitbucket_remote_urls() {
        assert_eq!(
            parse_bitbucket_slug_from_remote_url("git@bitbucket.org:acme"),
            Some(Err(()))
        );
        assert_eq!(
            parse_bitbucket_slug_from_remote_url("https://bitbucket.org/acme/api/extra"),
            Some(Err(()))
        );
    }

    #[test]
    fn finds_repo_root_from_nested_directory() {
        let root = write_repo(
            "nested",
            Some(
                r#"[remote "origin"]
    url = git@bitbucket.org:acme/api.git
"#,
            ),
        );
        let nested = nested_path(&root);

        let slug = detect_bitbucket_repo_slug_from_path(&nested).expect("slug should be detected");
        assert_eq!(slug, ("acme".into(), "api".into()));
    }

    #[test]
    fn fails_outside_git_repository() {
        let guard = cwd_lock().lock().expect("lock should work");
        let original = std::env::current_dir().expect("cwd should exist");
        let root = std::env::temp_dir().join(format!(
            "bb-git-none-{}-{}",
            std::process::id(),
            unique_id()
        ));
        fs::create_dir_all(&root).expect("temp dir should be created");
        std::env::set_current_dir(&root).expect("cwd should be changed");

        let err = super::detect_bitbucket_repo_slug_from_cwd().expect_err("should fail");
        assert!(err.to_string().contains("not inside a git repository"));

        std::env::set_current_dir(original).expect("cwd should be restored");
        drop(guard);
    }

    #[test]
    fn resolves_git_worktree_file() {
        let root = write_worktree_like_repo("file");
        let slug = detect_bitbucket_repo_slug_from_path(&root).expect("should resolve");
        assert_eq!(slug, ("acme".into(), "worktree".into()));
    }

    #[test]
    fn fails_when_git_config_missing() {
        let root = write_repo("missing-config", None);
        let err = detect_bitbucket_repo_slug_from_path(&root).expect_err("should fail");
        assert!(err.to_string().contains("failed to read"));
    }

    #[test]
    fn detects_https_remote_without_dot_git() {
        let root = write_repo(
            "https",
            Some(
                r#"[remote "origin"]
    url = https://bitbucket.org/acme/api
"#,
            ),
        );

        let slug = detect_bitbucket_repo_slug_from_path(&root).expect("slug should be detected");
        assert_eq!(slug, ("acme".into(), "api".into()));
    }

    #[test]
    fn ignores_non_bitbucket_remotes_when_one_bitbucket_remote_exists() {
        let root = write_repo(
            "mixed",
            Some(
                r#"[remote "github"]
    url = git@github.com:acme/api.git
[remote "origin"]
    url = git@bitbucket.org:acme/api.git
"#,
            ),
        );

        let slug = detect_bitbucket_repo_slug_from_path(&root).expect("slug should be detected");
        assert_eq!(slug, ("acme".into(), "api".into()));
    }

    #[test]
    fn fails_when_no_bitbucket_remote_exists() {
        let root = write_repo(
            "no-bitbucket",
            Some(
                r#"[remote "origin"]
    url = git@github.com:acme/api.git
"#,
            ),
        );

        let err = detect_bitbucket_repo_slug_from_path(&root).expect_err("should fail");
        assert!(err.to_string().contains("no Bitbucket remote"));
    }

    #[test]
    fn fails_when_multiple_bitbucket_remotes_exist() {
        let root = write_repo(
            "ambiguous",
            Some(
                r#"[remote "origin"]
    url = git@bitbucket.org:acme/api.git
[remote "upstream"]
    url = https://bitbucket.org/acme/api.git
"#,
            ),
        );

        let err = detect_bitbucket_repo_slug_from_path(&root).expect_err("should fail");
        let message = err.to_string();
        assert!(message.contains("multiple Bitbucket remotes"));
        assert!(message.contains("origin"));
        assert!(message.contains("upstream"));
    }

    #[test]
    fn fails_when_bitbucket_remote_is_malformed() {
        let root = write_repo(
            "malformed",
            Some(
                r#"[remote "origin"]
    url = https://bitbucket.org/acme/api/extra
"#,
            ),
        );

        let err = detect_bitbucket_repo_slug_from_path(&root).expect_err("should fail");
        assert!(err.to_string().contains("malformed Bitbucket remote"));
    }
}
