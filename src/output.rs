use std::io::{self, Write};

use crate::model::{
    PipelineDetailedJsonRow, PipelineDetailedRow, PipelineJsonRow, PipelineRow, PipelineStepJsonRow,
    PipelineStepRow, PullRequestCommentJsonRow, PullRequestCommentRow, PullRequestDetailedJsonRow,
    PullRequestDetailedRow, PullRequestJsonRow, PullRequestRow, RepoJsonRow, RepoRow,
};

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

pub fn write_pull_requests<W: Write>(
    mut writer: W,
    pull_requests: &[PullRequestRow],
) -> io::Result<()> {
    for pull_request in pull_requests {
        let label = format!(
            "{} {} {}",
            pull_request.id,
            pull_request.state,
            pull_request.title.as_deref().unwrap_or_default()
        );

        if let Some(url) = &pull_request.html_url {
            writeln!(writer, "{}", osc8_link(label.as_str(), url.as_str()))?;
        } else {
            writeln!(writer, "{label}")?;
        }
    }

    Ok(())
}

pub fn write_pull_requests_json<W: Write>(
    mut writer: W,
    pull_requests: &[PullRequestRow],
) -> io::Result<()> {
    let rows: Vec<PullRequestJsonRow> =
        pull_requests.iter().map(PullRequestJsonRow::from).collect();
    serde_json::to_writer(&mut writer, &rows)?;
    writeln!(writer)?;
    Ok(())
}

pub fn write_pull_request<W: Write>(
    mut writer: W,
    pr: &PullRequestDetailedRow,
) -> io::Result<()> {
    writeln!(
        writer,
        "PR #{}: {}",
        pr.common.id,
        pr.common.title.as_deref().unwrap_or("")
    )?;
    writeln!(writer, "State: {}", pr.common.state)?;
    writeln!(
        writer,
        "Author: {}",
        pr.common
            .author_display_name
            .as_deref()
            .unwrap_or("unknown")
    )?;
    writeln!(
        writer,
        "Branch: {} -> {}",
        pr.common.source_branch.as_deref().unwrap_or("unknown"),
        pr.common
            .destination_branch
            .as_deref()
            .unwrap_or("unknown")
    )?;
    if let Some(desc) = &pr.description {
        writeln!(writer)?;
        writeln!(writer, "{}", desc)?;
    }
    Ok(())
}

pub fn write_pull_request_json<W: Write>(
    mut writer: W,
    pr: &PullRequestDetailedRow,
) -> io::Result<()> {
    let row = PullRequestDetailedJsonRow::from(pr);
    serde_json::to_writer(&mut writer, &row)?;
    writeln!(writer)?;
    Ok(())
}

pub fn write_pull_request_comments<W: Write>(
    mut writer: W,
    comments: &[PullRequestCommentRow],
) -> io::Result<()> {
    for comment in comments {
        writeln!(
            writer,
            "[{}] {}",
            comment.author_display_name.as_deref().unwrap_or("unknown"),
            comment.content_raw.as_deref().unwrap_or("")
        )?;
        if let Some(inline) = &comment.inline {
            writeln!(writer, "  File: {}", inline.path)?;
            if let Some(line) = inline.to {
                writeln!(writer, "  Line: {}", line)?;
            }
        }
        writeln!(writer)?;
    }
    Ok(())
}

pub fn write_pull_request_comments_json<W: Write>(
    mut writer: W,
    comments: &[PullRequestCommentRow],
) -> io::Result<()> {
    let rows: Vec<PullRequestCommentJsonRow> =
        comments.iter().map(PullRequestCommentJsonRow::from).collect();
    serde_json::to_writer(&mut writer, &rows)?;
    writeln!(writer)?;
    Ok(())
}

pub fn write_pull_request_comment<W: Write>(
    mut writer: W,
    comment: &PullRequestCommentRow,
) -> io::Result<()> {
    writeln!(
        writer,
        "Comment created by {}",
        comment.author_display_name.as_deref().unwrap_or("unknown")
    )?;
    if let Some(content) = &comment.content_raw {
        writeln!(writer, "{}", content)?;
    }
    Ok(())
}

pub fn write_pull_request_comment_json<W: Write>(
    mut writer: W,
    comment: &PullRequestCommentRow,
) -> io::Result<()> {
    let row = PullRequestCommentJsonRow::from(comment);
    serde_json::to_writer(&mut writer, &row)?;
    writeln!(writer)?;
    Ok(())
}

fn osc8_link(label: &str, url: &str) -> String {
    format!("\u{1b}]8;;{url}\u{1b}\\{label}\u{1b}]8;;\u{1b}\\")
}

pub fn write_pipelines<W: Write>(mut writer: W, pipelines: &[PipelineRow]) -> io::Result<()> {
    for p in pipelines {
        let state_indicator = match p.state.as_str() {
            "COMPLETED" => "✓",
            "FAILED" => "✗",
            "IN_PROGRESS" => "⋯",
            "STOPPED" => "⊗",
            _ => "?",
        };

        let ref_display = p
            .target_ref_name
            .as_ref()
            .map(|r| format!(" {}", r))
            .unwrap_or_default();

        let label = format!(
            "{} #{} {}{}",
            state_indicator, p.build_number, p.state, ref_display
        );

        if !p.web_url.is_empty() {
            writeln!(writer, "{}", osc8_link(label.as_str(), &p.web_url))?;
        } else {
            writeln!(writer, "{label}")?;
        }
    }

    Ok(())
}

pub fn write_pipelines_json<W: Write>(mut writer: W, pipelines: &[PipelineRow]) -> io::Result<()> {
    let rows: Vec<PipelineJsonRow> = pipelines.iter().map(PipelineJsonRow::from).collect();
    serde_json::to_writer(&mut writer, &rows)?;
    writeln!(writer)?;
    Ok(())
}

pub fn write_pipeline<W: Write>(mut writer: W, pipeline: &PipelineDetailedRow) -> io::Result<()> {
    writeln!(writer, "Pipeline: #{}", pipeline.build_number)?;
    writeln!(writer, "UUID:     {}", pipeline.uuid)?;
    writeln!(writer, "State:    {}", pipeline.state)?;

    if let Some(ref_name) = &pipeline.target_ref_name {
        let ref_type = pipeline
            .target_ref_type
            .as_deref()
            .unwrap_or("unknown");
        writeln!(writer, "Ref:      {} ({})", ref_name, ref_type)?;
    }

    if let Some(creator) = &pipeline.creator {
        writeln!(writer, "Creator:  {}", creator)?;
    }

    if let Some(trigger) = &pipeline.trigger_type {
        writeln!(writer, "Trigger:  {}", trigger)?;
    }

    if let Some(created) = &pipeline.created_on {
        writeln!(writer, "Created:  {}", created)?;
    }

    if let Some(completed) = &pipeline.completed_on {
        writeln!(writer, "Completed: {}", completed)?;
    }

    if let Some(seconds) = pipeline.build_seconds_used {
        writeln!(writer, "Duration: {}s", seconds)?;
    }

    writeln!(writer, "URL:      {}", pipeline.web_url)?;

    Ok(())
}

pub fn write_pipeline_json<W: Write>(
    mut writer: W,
    pipeline: &PipelineDetailedRow,
) -> io::Result<()> {
    let row = PipelineDetailedJsonRow::from(pipeline);
    serde_json::to_writer(&mut writer, &row)?;
    writeln!(writer)?;
    Ok(())
}

pub fn write_pipeline_steps<W: Write>(
    mut writer: W,
    steps: &[PipelineStepRow],
) -> io::Result<()> {
    for (idx, step) in steps.iter().enumerate() {
        let state_indicator = match step.state.as_str() {
            "COMPLETED" => "✓",
            "FAILED" => "✗",
            "IN_PROGRESS" => "⋯",
            _ => "?",
        };

        let image_display = step
            .image_name
            .as_ref()
            .map(|i| format!(" [{}]", i))
            .unwrap_or_default();

        writeln!(
            writer,
            "{} Step {} {}{}",
            state_indicator,
            idx + 1,
            step.state,
            image_display
        )?;
    }

    Ok(())
}

pub fn write_pipeline_steps_json<W: Write>(
    mut writer: W,
    steps: &[PipelineStepRow],
) -> io::Result<()> {
    let rows: Vec<PipelineStepJsonRow> = steps.iter().map(PipelineStepJsonRow::from).collect();
    serde_json::to_writer(&mut writer, &rows)?;
    writeln!(writer)?;
    Ok(())
}

pub fn write_pipeline_step<W: Write>(
    mut writer: W,
    step: &PipelineStepRow,
) -> io::Result<()> {
    writeln!(writer, "Step:     {}", step.uuid)?;
    writeln!(writer, "State:    {}", step.state)?;

    if let Some(image) = &step.image_name {
        writeln!(writer, "Image:    {}", image)?;
    }

    if let Some(started) = &step.started_on {
        writeln!(writer, "Started:  {}", started)?;
    }

    if let Some(completed) = &step.completed_on {
        writeln!(writer, "Completed: {}", completed)?;
    }

    Ok(())
}

pub fn write_pipeline_step_json<W: Write>(
    mut writer: W,
    step: &PipelineStepRow,
) -> io::Result<()> {
    let row = PipelineStepJsonRow::from(step);
    serde_json::to_writer(&mut writer, &row)?;
    writeln!(writer)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        write_pull_requests, write_pull_requests_json, write_repositories, write_repositories_json,
    };
    use crate::model::{PullRequestRow, RepoRow};

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

    #[test]
    fn writes_one_pull_request_per_line() {
        let pull_requests = vec![
            PullRequestRow {
                workspace_slug: "acme".into(),
                repo_slug: "api".into(),
                id: 123,
                title: Some("Fix auth bug".into()),
                state: "OPEN".into(),
                html_url: Some("https://bitbucket.org/acme/api/pull-requests/123".into()),
                author_display_name: None,
                source_branch: None,
                destination_branch: None,
                draft: None,
                comment_count: None,
                task_count: None,
                created_on: None,
                updated_on: None,
            },
            PullRequestRow {
                workspace_slug: "acme".into(),
                repo_slug: "api".into(),
                id: 124,
                title: None,
                state: "UNKNOWN".into(),
                html_url: None,
                author_display_name: None,
                source_branch: None,
                destination_branch: None,
                draft: None,
                comment_count: None,
                task_count: None,
                created_on: None,
                updated_on: None,
            },
        ];

        let mut buf = Vec::new();
        write_pull_requests(&mut buf, &pull_requests).expect("write should succeed");

        let rendered = String::from_utf8(buf).expect("utf8 output");
        assert_eq!(
            rendered,
            concat!(
                "\u{1b}]8;;https://bitbucket.org/acme/api/pull-requests/123\u{1b}\\",
                "123 OPEN Fix auth bug",
                "\u{1b}]8;;\u{1b}\\\n",
                "124 UNKNOWN \n"
            )
        );
    }

    #[test]
    fn writes_curated_pull_request_json() {
        let pull_requests = vec![PullRequestRow {
            workspace_slug: "acme".into(),
            repo_slug: "api".into(),
            id: 123,
            title: Some("Fix auth bug".into()),
            state: "OPEN".into(),
            html_url: Some("https://bitbucket.org/acme/api/pull-requests/123".into()),
            author_display_name: Some("Alice".into()),
            source_branch: Some("feature/auth".into()),
            destination_branch: Some("main".into()),
            draft: Some(false),
            comment_count: Some(2),
            task_count: Some(1),
            created_on: Some("2026-04-15T12:00:00+00:00".into()),
            updated_on: Some("2026-04-16T12:00:00+00:00".into()),
        }];

        let mut buf = Vec::new();
        write_pull_requests_json(&mut buf, &pull_requests).expect("write should succeed");

        let rendered = String::from_utf8(buf).expect("utf8 output");
        assert_eq!(
            rendered,
            concat!(
                "[{\"workspace_slug\":\"acme\",\"repo_slug\":\"api\",\"id\":123,",
                "\"title\":\"Fix auth bug\",\"state\":\"OPEN\",",
                "\"author_display_name\":\"Alice\",",
                "\"source_branch\":\"feature/auth\",",
                "\"destination_branch\":\"main\",\"draft\":false,",
                "\"comment_count\":2,\"task_count\":1,",
                "\"created_on\":\"2026-04-15T12:00:00+00:00\",",
                "\"updated_on\":\"2026-04-16T12:00:00+00:00\"}]\n"
            )
        );
    }
}
