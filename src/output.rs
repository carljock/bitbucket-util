use std::io::{self, Write};

use crate::model::{RepoJsonRow, RepoRow};

pub fn write_repositories<W: Write>(mut writer: W, repos: &[RepoRow]) -> io::Result<()> {
    for repo in repos {
        writeln!(writer, "{}", repo.display_name())?;
    }

    Ok(())
}

pub fn write_repositories_json<W: Write>(mut writer: W, repos: &[RepoRow]) -> io::Result<()> {
    let rows: Vec<RepoJsonRow> = repos.iter().map(RepoJsonRow::from).collect();
    serde_json::to_writer(&mut writer, &rows)?;
    writeln!(writer)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{write_repositories, write_repositories_json};
    use crate::model::RepoRow;

    #[test]
    fn writes_one_repo_per_line() {
        let repos = vec![
            RepoRow {
                workspace_slug: "acme".to_string(),
                repo_slug: "api".to_string(),
                full_name: Some("acme/api".to_string()),
                name: None,
                description: None,
                is_private: None,
                language: None,
                updated_on: None,
                main_branch: None,
                clone_https: None,
                clone_ssh: None,
            },
            RepoRow {
                workspace_slug: "acme".to_string(),
                repo_slug: "web".to_string(),
                full_name: Some("acme/web".to_string()),
                name: None,
                description: None,
                is_private: None,
                language: None,
                updated_on: None,
                main_branch: None,
                clone_https: None,
                clone_ssh: None,
            },
        ];

        let mut buf = Vec::new();
        write_repositories(&mut buf, &repos).expect("write should succeed");

        let rendered = String::from_utf8(buf).expect("utf8 output");
        assert_eq!(rendered, "acme/api\nacme/web\n");
    }

    #[test]
    fn writes_compact_json_with_selected_fields() {
        let repos = vec![RepoRow {
            workspace_slug: "acme".to_string(),
            repo_slug: "api".to_string(),
            full_name: Some("acme/api".to_string()),
            name: Some("api".to_string()),
            description: Some("service".to_string()),
            is_private: Some(true),
            language: Some("rust".to_string()),
            updated_on: Some("2026-04-15T12:00:00+00:00".to_string()),
            main_branch: Some("main".to_string()),
            clone_https: Some("https://bitbucket.org/acme/api.git".to_string()),
            clone_ssh: Some("git@bitbucket.org:acme/api.git".to_string()),
        }];

        let mut buf = Vec::new();
        write_repositories_json(&mut buf, &repos).expect("write should succeed");

        let rendered = String::from_utf8(buf).expect("utf8 output");
        assert_eq!(
            rendered,
            concat!(
                "[{\"full_name\":\"acme/api\",\"workspace_slug\":\"acme\",",
                "\"repo_slug\":\"api\",\"description\":\"service\",\"visibility\":\"private\",",
                "\"language\":\"rust\",\"main_branch\":\"main\",",
                "\"updated_on\":\"2026-04-15T12:00:00+00:00\",",
                "\"clone_https\":\"https://bitbucket.org/acme/api.git\",",
                "\"clone_ssh\":\"git@bitbucket.org:acme/api.git\"}]\n"
            )
        );
    }
}
