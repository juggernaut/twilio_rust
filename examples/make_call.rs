extern crate twilio_rust;
extern crate tokio_core;
extern crate url;

use std::env;
use tokio_core::reactor::Core;
use twilio_rust::Client;
use twilio_rust::calls::{Calls, OutboundCallBuilder};
use url::Url;

fn main() {

    let from_num = env::var("FROM_NUMBER").expect("FROM_NUMBER must be set to a valid caller ID for your account");
    let to_num = env::var("TO_NUMBER").expect("TO_NUMBER must be set to the number you want to call");

    // Create the tokio event loop
    let mut core = Core::new().unwrap();

    // Create the twilio client
    let client = Client::new_from_env(&core.handle()).unwrap();

    let calls = Calls::new(&client);
    let cb_url = Url::parse("http://twimlets.com/echo?\
        Twiml=%3CResponse%3E%3CSay%3EHello+Rust.%3C%2FSay%3E%3C%2FResponse%3E")
        .unwrap();

    // Create the outbound call
    let outbound_call = OutboundCallBuilder::new(&from_num, &to_num , &cb_url).build();

    let work =     calls.make_call(&outbound_call);
    let call = core.run(work).unwrap();
    println!("Queued outbound call {}", call.sid);
}