extern crate hyper;

use std::str;
use ::Client;
use rfc2822;
use rfc2822::opt_deserialize;
use serde_json;
use chrono::prelude::*;
use futures::future;
use futures::{Future, Stream};
use hyper::{Body, Method, Request, Uri};
use hyper::header::{ContentType, ContentLength};
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
    //#[serde(with = "rfc2822")] pub date_created: DateTime<Utc>,
    #[serde(deserialize_with = "rfc2822::opt_deserialize")] pub date_created: Option<DateTime<Utc>>,
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
    status_callback_event: &'a [StatusCallbackEvent],
    send_digits: Option<&'a str>,
}

impl<'a> OutboundCall<'a> {
    pub fn new(from: &'a str, to: &'a str, url: &'a Url) -> OutboundCall<'a> {
       OutboundCall {
           from,
           to,
           twiml_source: TwimlSource::url(url),
           method: None,
           fallback_url: None,
           fallback_method: None,
           status_callback: None,
           status_callback_method: None,
           status_callback_event: &[],
           send_digits: None,
       }
    }

    fn with_fallback_url(&mut self, fallback_url: &'a Url) -> &mut Self {
        self.fallback_url = Some(fallback_url);
        self
    }
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

    pub fn make_call(&self, outbound_call: &OutboundCall) -> Box<Future<Item = Call, Error = ::TwilioError>> {
        let url_encoded = outbound_call.to_url_encoded();
        let uri = format!(
            "{}/Accounts/{}/Calls.json",
            ::BASE_URI,
            self.client.account_sid).parse().unwrap();
        let mut req = Request::new(Method::Post, uri);
        req.headers_mut().set(ContentType::form_url_encoded());
        req.headers_mut().set(ContentLength(url_encoded.len() as u64));
        req.set_body(url_encoded.into_bytes());
        self.client.get(req)
    }
}