use chrono::prelude::*;
use disfunction::Request;

pub trait RequestBuilder {
    fn build(token: &str) -> Option<Request>;
}

impl RequestBuilder for Request {
    fn build(token: &str) -> Option<Request> {
        Some(Request {
            token: token.to_string(),
            since: NaiveDate::from_ymd_opt(2025, 1, 1)?,
            until: Utc::now().date_naive(),
        })
    }
}
