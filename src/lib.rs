extern crate chrono;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio_core;
extern crate url;

use std::str;
use std::str::FromStr;
use std::option::Option;
use std::env;
use std::io;
use futures::{Future, Stream, future};
use tokio_core::reactor::Handle;
use hyper::client::{FutureResponse, HttpConnector};
use hyper_tls::HttpsConnector;
use hyper::{Body, Request, Uri, StatusCode};
use hyper::header::{Authorization, Basic};
use serde_json::value::Value;

pub const BASE_URI: &str = "https://api.twilio.com";

pub mod calls;
pub mod messages;
mod serde_helper;


pub struct Client {
    account_sid: String,
    auth_token: String,
    client: hyper::Client<HttpsConnector<HttpConnector>, Body>,
}

#[derive(Debug)]
pub enum TwilioError {
    Hyper(hyper::error::Error),
    Serde(serde_json::Error),
    BadResponse(hyper::Response),
    MalformedResponse,
}

pub struct Page<T> {
    pub items: Vec<T>,
    pub next_page_uri: Option<Uri>,
}

pub trait ToUrlEncoded {
    fn to_url_encoded(&self) -> String;
}

impl Client {
    pub fn new(account_sid: &str, auth_token: &str, handle: &Handle) -> Result<Client, io::Error> {
        let client = hyper::Client::configure()
            .connector(HttpsConnector::new(4, handle).unwrap())
            .build(handle);
        Ok(Client {
            account_sid: account_sid.to_string(),
            auth_token: auth_token.to_string(),
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

    fn make_req<'de, T>(&self, req: Request<Body>) -> Box<Future<Item = T, Error = TwilioError> + 'de>
    where T: 'de + serde::de::DeserializeOwned
    {
        let fut = self.send_request(req)
            .map_err(|err| TwilioError::Hyper(err))
            .and_then(|res| {
                match res.status() {
                    StatusCode::Ok | StatusCode::Created => future::ok(res),
                    _  => future::err(TwilioError::BadResponse(res)),
                }
            })
            .and_then(|res| {
                res.body().concat2().map_err(|err| TwilioError::Hyper(err))
            })
            .and_then(move |body| {
                let call_res = serde_json::from_slice(&body).map_err(|err| TwilioError::Serde(err));
                future::result(call_res)
            });
        Box::new(fut)
    }

    fn get_page(&self, req: Request<Body>) -> Box<Future<Item = Page<calls::Call>, Error = TwilioError>> {
        let fut = self.send_request(req)
            .map_err(|err| TwilioError::Hyper(err))
            .and_then(|res| {
                res.body().concat2().map_err(|err| TwilioError::Hyper(err))
            })
            .and_then(move |body| {
                let call_res: Result<Value, TwilioError> = serde_json::from_slice(&body)
                    .map_err(|err| TwilioError::Serde(err));
                let final_res = call_res.and_then(move|v| {
                    let next_page_uri = match v["next_page_uri"] {
                        Value::String(ref uri) => Uri::from_str(&format!("{}{}", BASE_URI, uri)).ok(),
                        _ => None,
                    };
                    v.get("calls")
                        .ok_or(TwilioError::MalformedResponse)
                        .and_then(move |v| v.as_array().ok_or(TwilioError::MalformedResponse))
                        .map(move|calls| {
                            let des_calls: Vec<calls::Call> = calls.to_owned().into_iter().map(move|c| {
                                let call: calls::Call = serde_json::from_value(c).unwrap(); // XXX: figure out if we should quit even if a single result is bad
                                call
                            }).collect();
                            Page {
                                items: des_calls,
                                next_page_uri,
                            }
                        })
                });
                future::result(final_res)
            });
        Box::new(fut)
    }
}
