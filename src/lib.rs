extern crate chrono;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio_core;

use std::str;
use std::option::Option;
use std::env;
use std::error::Error;
use std::process;
use std::thread;
use std::sync::mpsc;
use std::io::{self, Write};
use futures::{Future, Stream};
use tokio_core::reactor::{Core, Handle, Remote};
use chrono::prelude::*;
use hyper::client::{FutureResponse, HttpConnector};
use hyper_tls::HttpsConnector;
use hyper::{Body, Method, Request};
use hyper::header::{Authorization, Basic};

pub const BASE_URI: &str = "https://api.twilio.com/2010-04-01";

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

pub mod calls;
mod rfc2822;


pub struct Client {
    account_sid: String,
    auth_token: String,
    handle: Handle,
    client: hyper::Client<HttpsConnector<HttpConnector>, Body>,
}

impl Client {
    pub fn new(account_sid: &str, auth_token: &str, handle: &Handle) -> Result<Client, io::Error> {
        let client = hyper::Client::configure()
            .connector(HttpsConnector::new(4, handle).unwrap())
            .build(handle);
        Ok(Client {
            account_sid: account_sid.to_string(),
            auth_token: auth_token.to_string(),
            handle: handle.clone(),
            client: client,
        })
    }

    pub fn new_from_env(handle: &Handle) -> Result<Client, io::Error> {
        let account_sid = env::var("ACCOUNT_SID").expect("ACCOUNT_SID env variable must be set!");
        let auth_token = env::var("AUTH_TOKEN").expect("AUTH_TOKEN env variable must be set!");
        Self::new(&account_sid, &auth_token, handle)
    }

    fn send_request(&self, mut req: Request<Body>) -> FutureResponse {
        req.headers_mut().set(Authorization(Basic {
            username: self.account_sid.to_owned(),
            password: Some(self.auth_token.to_owned()),
        }));
        self.client.request(req)
    }
}
