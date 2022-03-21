use crate::event::{PushEvent, GitlabPushEvent, GitlabRepository, Repository};
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

pub fn process_gitlab_push_event(event: String) -> String {
    let result: GitlabPushEvent = serde_json::from_str(event.as_str()).unwrap();

    format_gitlab_commit_message(
        result.user_name,
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
    let branch = target.replace("refs/heads/", "");
    return format!(
        "[{}](https://github.com/{}) push to [{}]({}):[{}]({})\n{}",
        escape_markdown(&author),
        author,
        escape_markdown(repo_simple_name),
        repo.html_url,
        escape_markdown(&branch),
        format!("{}/tree/{}", repo.html_url, branch),
        concat
    );
}

fn format_gitlab_commit_message(author: String, repo: GitlabRepository, target: String, commits: Vec<Commit>) ->
String {
    let concat = commits
        .iter()
        .map(|c| format!("[➞]({}) {}\n", c.href, escape_markdown(&c.comment)))
        .fold("".to_string(), |mut acc, g| {
            acc.push_str(&g.to_string());
            acc
        });

    //let repo_simple_name = &repo.name[repo.name.find("/").unwrap() + 1..repo.name.len()];
    let branch = target.replace("refs/heads/", "");
    return format!(
        "[{}](https://github.com/{}) push to [{}]({}):[{}]({})\n{}",
        escape_markdown(&author),
        author,
        // escape_markdown(repo_simple_name),
        escape_markdown(&repo.name),
        repo.git_http_url,
        escape_markdown(&branch),
        format!("{}/tree/{}", repo.git_http_url, branch),
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
        let target = "refs/heads/my-branch";
        let commits = vec![Commit {
            href: "commit_url".to_string(),
            comment: "commit_comment"
                .to_string(),
        }];
        let result = format_commit_message("kviring".to_string(), repo, target.to_string(),
                                           commits);

        println!("{}", result);
        assert_eq!(result, "[kviring](https://github.com/kviring) push to [repo](repo_url):[my\\-branch]\
        (repo_url/tree/master)\
        \n[➞](commit_url) commit\\_comment\n")
    }
}