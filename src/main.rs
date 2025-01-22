mod request;
mod responder;

use disfunction::{run, Request};
use request::RequestBuilder;
use std::env;

#[tokio::main]
async fn main() {
    let token = env::var("GITHUB_TOKEN").unwrap();
    let req = Request::build(&token).unwrap();
    let tty = responder::Tty::new();
    run(req, tty).await
}
