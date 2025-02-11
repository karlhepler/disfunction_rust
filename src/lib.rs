use chrono::prelude::*;
use futures::{
    future,
    stream::{self, StreamExt},
};
use std::sync::Arc;
// use octocrab::{commits::CommitHandler, models::repos::RepoCommit, Error as OctocrabError};

#[derive(Debug)]
pub struct Request {
    pub token: String,
    pub since: DateTime<Utc>,
    pub until: DateTime<Utc>,
}

pub enum Response {
    Out(String),
    Err(String),
    Log(String),
}

pub trait Responder {
    fn send(&self, res: Response);
    fn out(&self, msg: String);
    fn err(&self, msg: String);
    fn log(&self, msg: String);
}

pub async fn run<T: Responder>(req: Request, res: Arc<T>) {
    res.out(format!("Hello, world! {:#?}", req));

    let octocrab = octocrab::instance().user_access_token(req.token);
    let octocrab = match octocrab {
        Ok(octocrab) => Arc::new(octocrab),
        Err(err) => {
            res.log(format!("{:#?}", err));
            res.err("error initializing GitHub client".to_string());
            return;
        }
    };

    // TODO(kjh): async pagination with smart rate limiting
    //
    // Reference:
    //  https://docs.github.com/en/rest/using-the-rest-api/rate-limits-for-the-rest-api?apiVersion=2022-11-28
    //  https://docs.github.com/en/rest/rate-limit/rate-limit?apiVersion=2022-11-28
    let repos = octocrab
        .current()
        .list_repos_for_authenticated_user()
        .send()
        .await;

    let repos = match repos {
        Ok(repos) => repos,
        Err(err) => {
            res.log(format!("{:#?}", err));
            res.err("error listing repos for authenticated user".to_string());
            return;
        }
    };

    let commits = stream::iter(repos.items)
        .map(|repo| (repo, Arc::clone(&octocrab), Arc::clone(&res)))
        // maybe use some sort of iterator caching mechanism
        // .cache_or(fn)
        .then(|(repo, octocrab, res)| async move {
            let commits = octocrab
                .repos_by_id(repo.id)
                .list_commits()
                .since(req.since)
                .until(req.until)
                .send()
                .await;

            match commits {
                Ok(commits) => Some(commits),
                Err(err) => {
                    res.log(format!("{:#?}", err));
                    res.err("error getting commit".to_string());
                    None
                }
            }
        })
        .filter(|opt| future::ready(opt.is_some())) // skip missing
        .flat_map(|opt| stream::iter(opt.unwrap())) //
        .collect::<Vec<_>>()
        .await; // now I need to get one random item from the list.
                //.into_iter().????

    res.out(format!("{:#?}", commits));
}
