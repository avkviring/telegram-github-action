use crate::event::{ReleaseEvent, Repository};
use crate::markdown::escape_markdown;

pub fn process_release_event(event: String) -> String {
    let result: ReleaseEvent = serde_json::from_str(event.as_str()).unwrap();

    format_release_message(
        result.release.author.login,
        result.release.author.html_url,
        result.repository,
        result.release.tag_name,
        result.release.html_url,
        result.release.name,
    )
}

fn format_release_message(
    author: String,
    author_url: String,
    repo: Repository,
    tag_name: Option<String>,
    release_url: String,
    name: Option<String>,
) -> String {
    return format!(
        "[{}]({}) release [{}@{}]({})\n{}",
        escape_markdown(&author),
        escape_markdown(&author_url),
        escape_markdown(&repo.full_name),
        escape_markdown(&tag_name.unwrap_or("".to_string())),
        escape_markdown(&release_url),
        escape_markdown(&name.unwrap_or("".to_string())),
    );
}
