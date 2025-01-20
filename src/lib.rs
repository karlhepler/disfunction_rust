use chrono::prelude::*;
use octocrab::{commits::CommitHandler, models::repos::RepoCommit, Error as OctocrabError};

#[derive(Debug)]
pub struct Request<'a> {
    pub token: &'a str,
    pub since: NaiveDate,
    pub until: NaiveDate,
}

pub struct Error {
    //
}

pub async fn run(req: Request<'_>) -> Option<Error> {
    println!("Hello, world! {:#?}", req);
    let octocrab = octocrab::instance().user_access_token(req.token)?;
    let repos = octocrab
        .current()
        .list_repos_for_authenticated_user()
        .send()
        .await;

    match repos {
        Ok(page) => {
            println!("page: {:#?}", page)
        }
        Err(msg) => eprintln!("error: {:#?}", msg),
    }

    // let _ = octocrab
    //     .commits("karlhepler", "disfunction")
    //     .list(req.since, req.until)
    //     .collect();
}

// trait ListCommits<'octo> {
//     async fn list(&self) -> Result<Vec<RepoCommit>, OctocrabError>;
// }

// impl<'octo> ListCommits<'_> for CommitHandler<'octo> {
//     async fn list(&self) -> Result<Vec<RepoCommit>, Error> {
//         todo!()
//     }
// }
