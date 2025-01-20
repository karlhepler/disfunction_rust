use chrono::prelude::*;
use disfunction::{run, Request};

fn main() {
    if let Some(req) = Request::build() {
        run(req);
        return;
    }

    eprintln!("[ERROR] invalid request")
}

trait RequestBuilder {
    fn build() -> Option<Request>;
}

impl RequestBuilder for Request {
    fn build() -> Option<Request> {
        Some(Request {
            since: NaiveDate::from_ymd_opt(2025, 01, 01)?,
            until: Utc::now().date_naive(),
        })
    }
}
