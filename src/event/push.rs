use crate::event::{PushEvent, Repository};
use crate::markdown::escape_markdown;

pub fn process_push_event(event: String) -> String {
    let result: PushEvent = serde_json::from_str(event.as_str()).unwrap();

    format_commit_message(
        result.pusher.name,
        result.repository,
        result
            .commits
            .iter()
            .map(|r| Commit {
                href: r.url.clone(),
                comment: r.message.clone(),
            })
            .collect(),
    )
}

struct Commit {
    href: String,
    comment: String,
}

fn format_commit_message(author: String, repo: Repository, commits: Vec<Commit>) -> String {
    let concat = commits
        .iter()
        .map(|c| format!("[➞]({}) {}\n", c.href, escape_markdown(&c.comment)))
        .fold("".to_string(), |mut acc, g| {
            acc.push_str(&g.to_string());
            acc
        });
    return format!(
        "[{}](https://github.com/{}) push to [{}]({})\n{}",
        escape_markdown(&author),
        author,
        escape_markdown(&repo.full_name),
        repo.html_url,
        concat
    );
}
