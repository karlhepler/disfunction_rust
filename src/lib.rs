use chrono::prelude::*;

#[derive(Debug)]
pub struct Request {
    pub since: NaiveDate,
    pub until: NaiveDate,
}

pub fn run(req: Request) {
    println!("Hello, world! {:#?}", req);
}
