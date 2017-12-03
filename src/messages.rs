extern crate hyper;

use ::{Client, ToUrlEncoded};
use url::{form_urlencoded, Url};
use chrono::prelude::*;
use serde_helper;
use hyper::{Method, Request};
use hyper::header::{ContentType, ContentLength};
use futures::Future;

pub struct Messages<'a> {
    client: &'a Client,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MessageStatus {
    Accepted,
    Queued,
    Sending,
    Sent,
    Failed,
    Delivered,
    Undelivered,
    Receiving,
    Received,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum MessageDirection {
    Inbound,
    OutboundApi,
    OutboundCall,
    OutboundReply,
}


#[derive(Deserialize)]
pub struct Message {
    pub sid: String,
    pub account_sid: String,
    pub messaging_service_sid: Option<String>,
    pub from: String,
    pub to: String,
    pub body: String,
    #[serde(deserialize_with = "serde_helper::deserialize_str_to_u32")] pub num_segments: Option<u32>,
    pub status: MessageStatus,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
    pub direction: MessageDirection,
    pub price: Option<String>,
    pub price_unit: Option<String>,
    #[serde(deserialize_with = "serde_helper::opt_deserialize")] pub date_created: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "serde_helper::opt_deserialize")] pub date_updated: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "serde_helper::opt_deserialize")] pub date_sent: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone)]
pub enum MessageBody<'a> {
    SMS(&'a str),
    MMS(&'a Url),
}

#[derive(Copy, Clone)]
pub enum MessageFrom<'a> {
    From(&'a str),
    MessagingServiceSid(&'a str),
}

pub struct OutboundMessage<'a> {
    to: &'a str,
    from: MessageFrom<'a>,
    body: MessageBody<'a>,
    status_callback: Option<&'a Url>,
    application_sid: Option<&'a str>,
    max_price: Option<&'a str>,
    provide_feedback: bool,
    validity_period: Option<u32>,
}

pub struct OutboundMessageBuilder<'a> {
    to: &'a str,
    from: MessageFrom<'a>,
    body: MessageBody<'a>,
    status_callback: Option<&'a Url>,
    application_sid: Option<&'a str>,
    max_price: Option<&'a str>,
    provide_feedback: bool,
    validity_period: Option<u32>,
}

impl<'a> OutboundMessageBuilder<'a> {
    pub fn new_sms(from: MessageFrom<'a>, to: &'a str, body: &'a str) -> OutboundMessageBuilder<'a> {
        OutboundMessageBuilder {
            from,
            to,
            body: MessageBody::SMS(body),
            status_callback: None,
            application_sid: None,
            max_price: None,
            provide_feedback: false,
            validity_period: None,
        }
    }

    pub fn new_mms(from: MessageFrom<'a>, to: &'a str, body: &'a Url) -> OutboundMessageBuilder<'a> {
        OutboundMessageBuilder {
            from,
            to,
            body: MessageBody::MMS(body),
            status_callback: None,
            application_sid: None,
            max_price: None,
            provide_feedback: false,
            validity_period: None,
        }
    }

    pub fn with_status_callback(&mut self, url: &'a Url) -> &mut Self {
        self.status_callback = Some(url);
        self
    }

    pub fn with_application_sid(&mut self, application_sid: &'a str) -> &mut Self {
        self.application_sid = Some(application_sid);
        self
    }

    pub fn with_max_price(&mut self, max_price: &'a str) -> &mut Self {
        self.max_price = Some(max_price);
        self
    }

    pub fn with_provide_feedback(&mut self, provide_feedback: bool) -> &mut Self {
        self.provide_feedback = provide_feedback;
        self
    }

    pub fn with_validity_period(&mut self, validity_period: u32) -> &mut Self {
        self.validity_period = Some(validity_period);
        self
    }

    pub fn build(&self) -> OutboundMessage<'a> {
        OutboundMessage {
            from: self.from,
            to: self.to,
            body: self.body,
            status_callback: self.status_callback,
            application_sid: self.application_sid,
            max_price: self.max_price,
            provide_feedback: self.provide_feedback,
            validity_period: self.validity_period,
        }
    }
}

impl<'a> ToUrlEncoded for OutboundMessage<'a> {

    fn to_url_encoded(&self) -> String {
        let mut encoder = form_urlencoded::Serializer::new(String::new());
        encoder.append_pair("To", self.to);

        let (name, value) = match self.from {
            MessageFrom::From(x) => ("From", x),
            MessageFrom::MessagingServiceSid(x) => ("MessagingServiceSid", x),
        };

        encoder.append_pair(name, value);

        let (name, value) = match self.body {
            MessageBody::SMS(x) => ("Body", x),
            MessageBody::MMS(x) => ("MediaUrl", x.as_str()),
        };

        encoder.append_pair(name, value);

        if let Some(url) = self.status_callback {
            encoder.append_pair("StatusCallback", url.as_str());
        }

        if let Some(application_sid) = self.application_sid {
            encoder.append_pair("ApplicationSid", application_sid);
        }

        if let Some(max_price) = self.max_price {
            encoder.append_pair("MaxPrice", max_price);
        }

        if self.provide_feedback {
            encoder.append_pair("ProvideFeedback", "true");
        }

        if let Some(validity_period) = self.validity_period {
            encoder.append_pair("ValidityPeriod", &validity_period.to_string());
        }


        encoder.finish()
    }
}

impl<'a> Messages<'a> {

    pub fn new(client: &'a Client) -> Messages {
        Messages { client }
    }

    pub fn send_message(&self, message: &'a OutboundMessage) -> Box<Future<Item = Message, Error = ::TwilioError>> {
        let encoded_params = message.to_url_encoded();
        let uri = format!(
            "{}/2010-04-01/Accounts/{}/Messages.json",
            ::BASE_URI,
            self.client.account_sid).parse().unwrap();
        let mut req = Request::new(Method::Post, uri);
        req.headers_mut().set(ContentType::form_url_encoded());
        req.headers_mut().set(ContentLength(encoded_params.len() as u64));
        req.set_body(encoded_params.into_bytes());
        self.client.make_req(req)
    }
}
