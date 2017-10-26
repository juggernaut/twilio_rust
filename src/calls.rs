extern crate hyper;

use std::str;
use ::Client;
use rfc2822;
use serde_json;
use chrono::prelude::*;
use futures::future;
use futures::{Future, Stream};
use hyper::{Body, Method, Request, Uri};
use hyper::error::Error;
use url::{form_urlencoded, Url};

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

pub enum TwimlSource<'a> {
    url(&'a Url),
    application_sid(&'a str),
}

pub enum CallbackMethod {
    post,
    get,
}

pub enum StatusCallbackEvent {
    initiated,
    ringing,
    answered,
    completed,
}

pub trait IntoUrlEncoded {
    fn to_url_encoded(&self) -> String;
}

pub struct OutboundCall<'a> {
    from: &'a str,
    to: &'a str,
    twiml_source: TwimlSource<'a>,
    method: Option<CallbackMethod>,
    fallback_url: Option<&'a Url>,
    fallback_method: Option<CallbackMethod>,
    status_callback: Option<&'a Url>,
    status_callback_method: Option<CallbackMethod>,
    status_callback_event: Vec<StatusCallbackEvent>,
    send_digits: &'a str,
}

impl<'a> IntoUrlEncoded for OutboundCall<'a> {

    fn to_url_encoded(&self) -> String {
        let mut encoder = form_urlencoded::Serializer::new(String::new());
        encoder.append_pair("From", self.from);
        encoder.append_pair("To", self.to);
        let (name, value) = match self.twiml_source {
            TwimlSource::url(x) => ("Url", x.as_str()),
            TwimlSource::application_sid(x) => ("ApplicationSid", x),
        };
        let _ = match self.method {
            Some(ref x) => {
                encoder.append_pair("Method", match *x {
                    CallbackMethod::post => "POST",
                    CallbackMethod::get => "GET",
                });
                ()
            }
            None => ()
        };
        encoder.append_pair(name, value);
        encoder.finish()
    }
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