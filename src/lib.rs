use chrono::prelude::*;

#[derive(Debug)]
pub struct Request {
    pub since: NaiveDate,
    pub until: NaiveDate, // AllowedRepos github.RepoAllowList
                          // AllowedFiles github.FileAllowList
}

pub fn run(req: Request) {
    println!("Hello, world! {:?}", req);
}
