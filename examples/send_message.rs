extern crate twilio_rust;
extern crate tokio_core;

use std::env;
use tokio_core::reactor::Core;
use twilio_rust::Client;
use twilio_rust::messages::{Messages, OutboundMessageBuilder, MessageFrom};

fn main() {

    let from_num = env::var("FROM_NUMBER").expect("FROM_NUMBER must be set to a valid caller ID for your account");
    let to_num = env::var("TO_NUMBER").expect("TO_NUMBER must be set to the number you want to send the message to");

    // Create the tokio event loop
    let mut core = Core::new().unwrap();

    // Create the twilio client
    let client = Client::new_from_env(&core.handle()).unwrap();

    let messages = Messages::new(&client);

    // Create the outbound SMS
    let outbound_sms = OutboundMessageBuilder::new_sms(
        MessageFrom::From(&from_num),
        &to_num,
        "Hello from Rust!"
    ).build();

    let work =     messages.send_message(&outbound_sms);
    let sms = core.run(work).unwrap();
    println!("Queued outbound SMS {}", sms.sid);
}
