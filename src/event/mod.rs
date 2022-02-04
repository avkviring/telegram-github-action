pub mod push;
pub mod release;

#[derive(Deserialize, Debug)]
pub struct PushEvent {
    pub after: String,
    pub base_ref: Option<String>,
    pub r#ref: String,
    pub before: String,
    pub commits: Vec<Commit>,
    pub compare: String,
    pub pusher: Pusher,
    pub repository: Repository,
}

#[derive(Deserialize, Debug)]
pub struct ReleaseEvent {
    pub action: String,
    pub release: Release,
    pub repository: Repository,
}

#[derive(Deserialize, Debug)]
pub struct Release {
    pub author: ReleaseAuthor,
    pub tag_name: Option<String>,
    pub name: Option<String>,
    pub body: Option<String>,
    pub html_url: String,
}

#[derive(Deserialize, Debug)]
pub struct Commit {
    pub author: CommitAuthor,
    pub committer: CommitAuthor,
    pub distinct: bool,
    pub id: String,
    pub message: String,
    pub timestamp: String,
    pub tree_id: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct CommitAuthor {
    pub email: String,
    pub name: String,
    pub username: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ReleaseAuthor {
    pub login: String,
    pub html_url: String,
}

#[derive(Deserialize, Debug)]
pub struct Pusher {
    pub email: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Repository {
    pub full_name: String,
    pub html_url: String,
}
