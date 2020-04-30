#[derive(Deserialize, Debug)]
pub struct PushEvent {
    pub after: String,
    pub base_ref: Option<String>,
    pub before: String,
    pub commits: Vec<Commit>,
    pub compare: String,
    pub pusher: Pusher,
    pub repository: Repository,
}

#[derive(Deserialize, Debug)]
pub struct Commit {
    pub author: Author,
    pub committer: Author,
    pub distinct: bool,
    pub id: String,
    pub message: String,
    pub timestamp: String,
    pub tree_id: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Author {
    pub email: String,
    pub name: String,
    pub username: String,
}

#[derive(Deserialize, Debug)]
pub struct Pusher {
    pub email: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Repository {
    pub name: String,
    pub html_url: String,
}
