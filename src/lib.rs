extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio_core;

use std::env;
use std::error::Error;
use std::process;
use std::thread;
use std::sync::mpsc;
use std::io::{self, Write};
use futures::{Future, Stream};
use tokio_core::reactor::{Core, Handle, Remote};
use hyper::client::{FutureResponse, HttpConnector};
use hyper_tls::HttpsConnector;
use hyper::{Body, Method, Request};
use hyper::header::{Authorization, Basic};

const BASE_URI: &str = "https://api.twilio.com/2010-04-01";

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

#[derive(Serialize, Deserialize)]
struct Call {
    sid: String,
}

trait Resource {
    fn get(&self, client: &Client) -> Box<Future<Item = Self, Error = hyper::error::Error>>;
    //fn post(&self) -> Box<Future<Item = Self, Error = io::Error>>;
}

impl Resource for Call {
    fn get(&self, client: &Client) -> Box<Future<Item = Self, Error = hyper::error::Error>> {
        let fut = client.send_request().and_then(|res| {
            println!("Response: {}", res.status());
            res.body().concat2()
        })
        .map(move |body| {
            /*
            let call_res = serde_json::from_slice(&body).map_err(|e| {
                io::Error::new(
                    io::ErrorKind::Other,
                    e)
            });
            */
            let call_res = serde_json::from_slice(&body).unwrap();
            call_res
        });
        Box::new(fut)
    }
}

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

    pub fn send_request(&self) -> FutureResponse {
        let uri = format!(
            "{}/Accounts/{}/Calls/{}.json",
            BASE_URI,
            self.account_sid,
            "CA166b2ee048446651bfccad9cdba48418"
        ).parse()
            .unwrap();
        let mut req: Request<Body> = Request::new(Method::Get, uri);
        req.headers_mut().set(Authorization(Basic {
            username: self.account_sid.to_owned(),
            password: Some(self.auth_token.to_owned()),
        }));
        let get_task = self.client.request(req);
        return get_task;
    }
}
