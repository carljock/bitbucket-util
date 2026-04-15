use serde::Serialize;

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
    pub language: Option<String>,
    pub updated_on: Option<String>,
    pub main_branch: Option<String>,
    pub clone_https: Option<String>,
    pub clone_ssh: Option<String>,
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

    pub fn visibility(&self) -> Option<&'static str> {
        match self.is_private {
            Some(true) => Some("private"),
            Some(false) => Some("public"),
            None => None,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub struct RepoJsonRow {
    pub full_name: Option<String>,
    pub workspace_slug: String,
    pub repo_slug: String,
    pub description: Option<String>,
    pub visibility: Option<&'static str>,
    pub language: Option<String>,
    pub main_branch: Option<String>,
    pub updated_on: Option<String>,
    pub clone_https: Option<String>,
    pub clone_ssh: Option<String>,
}

impl From<&RepoRow> for RepoJsonRow {
    fn from(value: &RepoRow) -> Self {
        Self {
            full_name: value.full_name.clone(),
            workspace_slug: value.workspace_slug.clone(),
            repo_slug: value.repo_slug.clone(),
            description: value.description.clone(),
            visibility: value.visibility(),
            language: value.language.clone(),
            main_branch: value.main_branch.clone(),
            updated_on: value.updated_on.clone(),
            clone_https: value.clone_https.clone(),
            clone_ssh: value.clone_ssh.clone(),
        }
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
            language: None,
            updated_on: None,
            main_branch: None,
            clone_https: None,
            clone_ssh: None,
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
    fn visibility_maps_private_flag() {
        let mut row = repo("acme", "app", Some("acme/app"));
        row.is_private = Some(true);
        assert_eq!(row.visibility(), Some("private"));

        row.is_private = Some(false);
        assert_eq!(row.visibility(), Some("public"));

        row.is_private = None;
        assert_eq!(row.visibility(), None);
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
