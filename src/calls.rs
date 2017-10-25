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

#[derive(Debug)]
pub enum TwilioError {
    Hyper(hyper::error::Error),
    Serde(serde_json::Error),
}

impl<'a> Calls<'a> {

    pub fn new(client: &Client) -> Calls {
        Calls { client }
    }

    pub fn get_call(
        &self,
        call_sid: &str,
    ) -> Box<Future<Item = Call, Error = TwilioError>> {
        let uri = format!(
            "{}/Accounts/{}/Calls/{}.json",
            ::BASE_URI,
            self.client.account_sid,
            call_sid
        ).parse()
            .unwrap();
        let mut req: Request<Body> = Request::new(Method::Get, uri);
        let fut = self.client.send_request(req)
            .map_err(|err| TwilioError::Hyper(err))
            .and_then(|res| {
                println!("Response: {}", res.status());
                res.body().concat2().map_err(|err| TwilioError::Hyper(err))
            })
            .and_then(move |body| {
                let debug_str = str::from_utf8(&body).unwrap();
                println!("DEBUG: body is {}", debug_str);
                let call_res = serde_json::from_slice(&body).map_err(|err| TwilioError::Serde(err));
                future::result(call_res)
            });
        Box::new(fut)
    }
}