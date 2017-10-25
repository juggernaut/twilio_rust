extern crate hyper;

use std::str;
use ::Client;
use rfc2822;
use serde_json;
use chrono::prelude::*;
use futures::future;
use futures::{Future, Stream};
use hyper::{Body, Method, Request};
use hyper::error::Error;

pub struct Calls<'a> {
    client: &'a Client,
}

#[derive(Serialize, Deserialize)]
pub struct Call {
    pub sid: String,
    pub account_sid: String,
    pub parent_call_sid: Option<String>,
    #[serde(with = "rfc2822")] pub date_created: DateTime<Utc>,
}

impl<'a> Calls<'a> {

    pub fn new(client: &Client) -> Calls {
        Calls { client }
    }

    pub fn get_call(
        &self,
        call_sid: &str,
    ) -> Box<Future<Item = Call, Error = ::TwilioError>> {
        let uri = format!(
            "{}/Accounts/{}/Calls/{}.json",
            ::BASE_URI,
            self.client.account_sid,
            call_sid
        ).parse()
            .unwrap();
        let mut req: Request<Body> = Request::new(Method::Get, uri);
        self.client.get(req)
    }
}