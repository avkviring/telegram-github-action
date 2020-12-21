use crate::github::{PushEvent, Repository};

pub fn process_push_event(event: String) -> String {
    let result: PushEvent = serde_json::from_str(event.as_str()).unwrap();

    let message = format_commit_message(
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
    );
    return message;
}

struct Commit {
    href: String,
    comment: String,
}

pub fn escape_markdown(source: &String) -> String {
    let mut result = source.clone();
    let chars = vec![
        "*", "_", "{", "}", "[", "]", "(", ")", "#", "+", "-", ".", "!", ">",
    ];
    chars.into_iter().for_each(|c| {
        let to = format!("\\{}", c);
        result = result.replace(c, to.as_str());
    });
    result
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
        "[{}](https://github.com/{}) push to [{}]({})\n {}",
        escape_markdown(&author),
        author,
        escape_markdown(&repo.name),
        repo.html_url,
        concat
    );
}
