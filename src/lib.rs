use chrono::prelude::*;
use octocrab::models::repos::RepoCommit;

#[derive(Debug)]
pub struct Request {
    pub since: NaiveDate,
    pub until: NaiveDate,
}

pub fn run(req: Request) {
    println!("Hello, world! {:#?}", req);
}

trait ListCommits {
    async fn list(&self) -> Result<Vec<RepoCommit>>;
}
