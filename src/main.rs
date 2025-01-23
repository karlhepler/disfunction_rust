mod request;
mod responder;

use disfunction::{run, Request};
use request::RequestBuilder;
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let token = env::var("GITHUB_TOKEN").unwrap();
    let req = Request::build(&token).unwrap();
    let tty = responder::Tty::new();
    run(req, Arc::new(tty)).await
}
