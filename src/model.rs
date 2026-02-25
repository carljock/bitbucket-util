#[derive(Debug, Clone, Eq, PartialEq)]
pub struct WorkspaceRef {
    pub slug: String,
    pub uuid: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RepoRow {
    pub workspace_slug: String,
    pub repo_slug: String,
    pub full_name: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_private: Option<bool>,
}

impl RepoRow {
    pub fn display_name(&self) -> String {
        if let Some(full_name) = &self.full_name {
            if !full_name.trim().is_empty() {
                return full_name.clone();
            }
        }

        format!("{}/{}", self.workspace_slug, self.repo_slug)
    }
}

pub fn normalize_repositories(mut repos: Vec<RepoRow>) -> Vec<RepoRow> {
    repos.sort_by(|a, b| {
        a.workspace_slug
            .cmp(&b.workspace_slug)
            .then(a.repo_slug.cmp(&b.repo_slug))
            .then(a.full_name.cmp(&b.full_name))
    });

    repos.dedup_by(|a, b| a.workspace_slug == b.workspace_slug && a.repo_slug == b.repo_slug);
    repos
}

#[cfg(test)]
mod tests {
    use super::{RepoRow, normalize_repositories};

    fn repo(workspace: &str, slug: &str, full_name: Option<&str>) -> RepoRow {
        RepoRow {
            workspace_slug: workspace.to_string(),
            repo_slug: slug.to_string(),
            full_name: full_name.map(str::to_owned),
            name: None,
            description: None,
            is_private: None,
        }
    }

    #[test]
    fn display_name_prefers_full_name() {
        let row = repo("acme", "app", Some("acme/app"));
        assert_eq!(row.display_name(), "acme/app");
    }

    #[test]
    fn display_name_falls_back_to_workspace_and_slug() {
        let row = repo("acme", "app", None);
        assert_eq!(row.display_name(), "acme/app");
    }

    #[test]
    fn normalize_repositories_sorts_and_deduplicates() {
        let rows = vec![
            repo("beta", "service", Some("beta/service")),
            repo("acme", "app", Some("acme/app")),
            repo("acme", "app", None),
        ];

        let normalized = normalize_repositories(rows);

        assert_eq!(normalized.len(), 2);
        assert_eq!(normalized[0].workspace_slug, "acme");
        assert_eq!(normalized[0].repo_slug, "app");
        assert_eq!(normalized[1].workspace_slug, "beta");
        assert_eq!(normalized[1].repo_slug, "service");
    }
}
