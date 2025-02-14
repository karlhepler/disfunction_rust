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

pub enum Message<T> {
    Out(T),
    Err(T),
    Log(T),
}

pub trait Messenger {
    fn send_to_requestor<T>(&self, msg: Message<T>);
    fn send_to_github<T>(&self, msg: Message<T>);
}

pub async fn run<T: Messenger>(req: Request, msg: Arc<T>) {
    msg.send_to_requestor(Message::Out(format!(
        "Hello! This is your request.\n{:#?}",
        req
    )));

    let octocrab = octocrab::instance().user_access_token(req.token);
    let octocrab = match octocrab {
        Ok(octocrab) => Arc::new(octocrab),
        Err(err) => {
            msg.send_to_requestor(Message::Log(format!("[ERROR] {:#?}", err)));
            msg.send_to_requestor(Message::Err("error initializing GitHub client".to_string()));
            return;
        }
    };

    // TODO(karlhepler): I think I need to do this.
    // According to GitHub docs, they recommend using a request queue to make requests to GitHub
    // API. I think that's a great idea. The queue can intercept the response from GitHub to check
    // if it's exceeding the rate limit. It sleeps for the remaining time until the recommended
    // time period passes. It's like clamping fps in a game loop.
    // I think I can just make functions for each request and use queue::Queue to move them through
    // with a loop and a sleep function. At the beginning of the loop, populate the queue. Then
    // immediately dequeue and get that value.

    let per_page = 100; // 100 is the maximum allowed value
    let mut current_page = 1; // default value for page is 1

    // TODO(kjh): async pagination with smart rate limiting
    //
    // Reference:
    //  https://docs.github.com/en/rest/using-the-rest-api/rate-limits-for-the-rest-api?apiVersion=2022-11-28
    //  https://docs.github.com/en/rest/rate-limit/rate-limit?apiVersion=2022-11-28
    //  https://docs.github.com/en/rest/repos/repos?apiVersion=2022-11-28#list-repositories-for-the-authenticated-user
    let page_of_repos_result = octocrab
        .current()
        .list_repos_for_authenticated_user()
        .per_page(per_page)
        .page(current_page)
        .visibility("all")
        .sort("created") // created is deterministict and can never change. I can use this for caching.
        .direction("asc") // this must not change. It is essential for caching.
        .send()
        .await;

    let page_of_repos = match page_of_repos_result {
        Ok(page_of_repos) => page_of_repos,
        Err(err) => {
            res.log(format!("{:#?}", err));
            res.err(format!(
                "error listing page {} of repos for authenticated user",
                current_page
            ));
            return;
        }
    };

    page_of_repos.last();

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
