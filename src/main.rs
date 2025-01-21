use chrono::prelude::*;
use disfunction::{run, Request};
use std::env;

#[tokio::main]
async fn main() {
    let token = env::var("GITHUB_TOKEN").unwrap();
    let req = Request::build(&token).unwrap();
    if let Some(err) = run(req).await {
        eprintln!("{:#?}", err)
    }
}

trait RequestBuilder {
    fn build(token: &str) -> Option<Request>;
}

impl RequestBuilder for Request {
    fn build(token: &str) -> Option<Request> {
        Some(Request {
            token: token.to_string(),
            since: NaiveDate::from_ymd_opt(2025, 01, 01)?,
            until: Utc::now().date_naive(),
        })
    }
}
