use chrono::prelude::*;
// use octocrab::{commits::CommitHandler, models::repos::RepoCommit, Error as OctocrabError};

#[derive(Debug)]
pub struct Request {
    pub token: String,
    pub since: NaiveDate,
    pub until: NaiveDate,
}

#[derive(Debug)]
pub struct Error {
    pub err: Box<dyn std::error::Error>,
}

pub async fn run(req: Request) -> Option<Error> {
    println!("Hello, world! {:#?}", req);

    let octocrab = octocrab::instance().user_access_token(req.token);
    let octocrab = match octocrab {
        Ok(octocrab) => octocrab,
        Err(err) => return Some(Error { err: Box::new(err) }),
    };

    let repos = octocrab
        .current()
        .list_repos_for_authenticated_user()
        .send()
        .await;
    let repos = match repos {
        Ok(repos) => repos,
        Err(err) => return Some(Error { err: Box::new(err) }),
    };

    eprintln!("{:#?}", repos);

    // repos.items.into_iter().

    // let _ = octocrab
    //     .commits("karlhepler", "disfunction")
    //     .list(req.since, req.until)
    //     .collect();

    None
}

// trait ListCommits<'octo> {
//     async fn list(&self) -> Result<Vec<RepoCommit>, OctocrabError>;
// }

// impl<'octo> ListCommits<'_> for CommitHandler<'octo> {
//     async fn list(&self) -> Result<Vec<RepoCommit>, Error> {
//         todo!()
//     }
// }
