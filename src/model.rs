use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PullRequestRow {
    pub workspace_slug: String,
    pub repo_slug: String,
    pub id: i32,
    pub title: Option<String>,
    pub state: String,
    pub html_url: Option<String>,
    pub author_display_name: Option<String>,
    pub source_branch: Option<String>,
    pub destination_branch: Option<String>,
    pub draft: Option<bool>,
    pub comment_count: Option<i32>,
    pub task_count: Option<i32>,
    pub created_on: Option<String>,
    pub updated_on: Option<String>,
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

#[derive(Debug, Clone, Eq, PartialEq, Serialize, JsonSchema)]
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

#[derive(Debug, Clone, Eq, PartialEq, Serialize, JsonSchema)]
pub struct PullRequestJsonRow {
    pub workspace_slug: String,
    pub repo_slug: String,
    pub id: i32,
    pub title: Option<String>,
    pub state: String,
    pub author_display_name: Option<String>,
    pub source_branch: Option<String>,
    pub destination_branch: Option<String>,
    pub draft: Option<bool>,
    pub comment_count: Option<i32>,
    pub task_count: Option<i32>,
    pub created_on: Option<String>,
    pub updated_on: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, JsonSchema)]
pub struct PullRequestDetailedJsonRow {
    #[serde(flatten)]
    pub common: PullRequestJsonRow,
    pub description: Option<String>,
    pub reviewers: Vec<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, JsonSchema)]
pub struct PullRequestCommentJsonRow {
    pub id: i64,
    pub author_display_name: Option<String>,
    pub content_raw: Option<String>,
    pub inline: Option<PullRequestInlineComment>,
    pub created_on: Option<String>,
    pub updated_on: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct PullRequestInlineComment {
    pub path: String,
    pub from: Option<i32>,
    pub to: Option<i32>,
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PullRequestDetailedRow {
    pub common: PullRequestRow,
    pub description: Option<String>,
    pub reviewers: Vec<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PullRequestCommentRow {
    pub id: i64,
    pub author_display_name: Option<String>,
    pub content_raw: Option<String>,
    pub inline: Option<PullRequestInlineComment>,
    pub created_on: Option<String>,
    pub updated_on: Option<String>,
}

impl From<&PullRequestRow> for PullRequestJsonRow {
    fn from(value: &PullRequestRow) -> Self {
        Self {
            workspace_slug: value.workspace_slug.clone(),
            repo_slug: value.repo_slug.clone(),
            id: value.id,
            title: value.title.clone(),
            state: value.state.clone(),
            author_display_name: value.author_display_name.clone(),
            source_branch: value.source_branch.clone(),
            destination_branch: value.destination_branch.clone(),
            draft: value.draft,
            comment_count: value.comment_count,
            task_count: value.task_count,
            created_on: value.created_on.clone(),
            updated_on: value.updated_on.clone(),
        }
    }
}

impl From<&PullRequestDetailedRow> for PullRequestDetailedJsonRow {
    fn from(value: &PullRequestDetailedRow) -> Self {
        Self {
            common: PullRequestJsonRow::from(&value.common),
            description: value.description.clone(),
            reviewers: value.reviewers.clone(),
        }
    }
}

impl From<&PullRequestCommentRow> for PullRequestCommentJsonRow {
    fn from(value: &PullRequestCommentRow) -> Self {
        Self {
            id: value.id,
            author_display_name: value.author_display_name.clone(),
            content_raw: value.content_raw.clone(),
            inline: value.inline.clone(),
            created_on: value.created_on.clone(),
            updated_on: value.updated_on.clone(),
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
    use super::{PullRequestJsonRow, PullRequestRow, RepoRow, normalize_repositories};

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

    #[test]
    fn pull_request_json_row_preserves_selected_fields() {
        let row = PullRequestRow {
            workspace_slug: "acme".into(),
            repo_slug: "api".into(),
            id: 42,
            title: Some("Improve auth".into()),
            state: "OPEN".into(),
            html_url: Some("https://bitbucket.org/acme/api/pull-requests/42".into()),
            author_display_name: Some("Alice".into()),
            source_branch: Some("feature/auth".into()),
            destination_branch: Some("main".into()),
            draft: Some(false),
            comment_count: Some(3),
            task_count: Some(1),
            created_on: Some("2026-04-15T12:00:00+00:00".into()),
            updated_on: Some("2026-04-16T12:00:00+00:00".into()),
        };

        let json_row = PullRequestJsonRow::from(&row);

        assert_eq!(json_row.id, 42);
        assert_eq!(json_row.state, "OPEN");
        assert_eq!(json_row.author_display_name.as_deref(), Some("Alice"));
        assert_eq!(json_row.source_branch.as_deref(), Some("feature/auth"));
        assert_eq!(json_row.destination_branch.as_deref(), Some("main"));
    }
}
