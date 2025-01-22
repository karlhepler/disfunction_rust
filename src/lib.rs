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

pub enum Response {
    Out(String),
    Err(String),
    Log(String),
}

pub trait Responder {
    fn send(&self, res: Response);
}

pub async fn run<T: Responder>(req: Request, res: T) {
    res.send(Response::Out(format!("Hello, world! {:#?}", req)));

    let octocrab = octocrab::instance().user_access_token(req.token);
    let octocrab = match octocrab {
        Ok(octocrab) => octocrab,
        Err(err) => {
            res.send(Response::Log(err.to_string()));
            res.send(Response::Err(
                "error initializing GitHub client".to_string(),
            ));
            return;
        }
    };

    let repos = octocrab
        .current()
        .list_repos_for_authenticated_user()
        .send()
        .await;
    let _ = match repos {
        Ok(repos) => repos,
        Err(err) => {
            res.send(Response::Log(err.to_string()));
            res.send(Response::Err(
                "error listing repos for authenticated user".to_string(),
            ));
            return;
        }
    };

    // res.send(Response::Out(format!("{:#?}", repos)))

    // repos.items.into_iter().

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
