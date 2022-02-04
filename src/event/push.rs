use crate::event::{PushEvent, Repository};
use crate::markdown::escape_markdown;

pub fn process_push_event(event: String) -> String {
    let result: PushEvent = serde_json::from_str(event.as_str()).unwrap();


    format_commit_message(
        result.pusher.name,
        result.repository,
        result.r#ref,
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

fn format_commit_message(author: String, repo: Repository, target: String, commits: Vec<Commit>) ->
String {
    let concat = commits
        .iter()
        .map(|c| format!("[➞]({}) {}\n", c.href, escape_markdown(&c.comment)))
        .fold("".to_string(), |mut acc, g| {
            acc.push_str(&g.to_string());
            acc
        });

    let repo_simple_name = &repo.full_name[repo.full_name.find("/").unwrap() + 1..repo.full_name.len
    ()];
    return format!(
        "[{}](https://github.com/{}) push to [{}\\({}\\)]({})\n{}",
        escape_markdown(&author),
        author,
        escape_markdown(repo_simple_name),
        target.replace("refs/heads/", ""),
        repo.html_url,
        concat
    );
}


#[cfg(test)]
mod tests {
    use crate::event::push::{Commit, format_commit_message};
    use crate::event::Repository;

    #[test]
    pub fn test_commit_message() {
        let repo = Repository {
            full_name: "command/repo".to_string(),
            html_url: "repo_url"
                .to_string
                (),
        };
        let target = "refs/heads/master";
        let commits = vec![Commit {
            href: "commit_url".to_string(),
            comment: "commit_comment"
                .to_string(),
        }];
        let result = format_commit_message("kviring".to_string(), repo, target.to_string(),
                                           commits);

        println!("{}", result);
        assert_eq!(result, "[kviring](https://github.com/kviring) push to [repo\\(master\\)]\
        (repo_url)\n[➞](commit_url) commit\\_comment\n")
    }
}