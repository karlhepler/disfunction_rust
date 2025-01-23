use chrono::prelude::*;
use disfunction::Request;

pub trait RequestBuilder {
    fn build(token: &str) -> Option<Request>;
}

impl RequestBuilder for Request {
    fn build(token: &str) -> Option<Request> {
        Some(Request {
            token: token.to_string(),
            since: Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap(),
            until: Utc::now(),
        })
    }
}
