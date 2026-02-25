use std::io::{self, Write};

use crate::model::RepoRow;

pub fn write_repositories<W: Write>(mut writer: W, repos: &[RepoRow]) -> io::Result<()> {
    for repo in repos {
        writeln!(writer, "{}", repo.display_name())?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::write_repositories;
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
            },
            RepoRow {
                workspace_slug: "acme".to_string(),
                repo_slug: "web".to_string(),
                full_name: Some("acme/web".to_string()),
                name: None,
                description: None,
                is_private: None,
            },
        ];

        let mut buf = Vec::new();
        write_repositories(&mut buf, &repos).expect("write should succeed");

        let rendered = String::from_utf8(buf).expect("utf8 output");
        assert_eq!(rendered, "acme/api\nacme/web\n");
    }
}
