extern crate chrono;
extern crate futures;
extern crate hyper;
extern crate tokio_core;

extern crate twilio_rust;
extern crate url;

use std::{env, thread, time};
use std::process;
use std::io::{self, Write};
use futures::{Future, Stream};
use futures::future;
use twilio_rust::{Client, Page};
use twilio_rust::calls::{Calls, OutboundCall, OutboundCallBuilder, CallbackMethod};
use twilio_rust::messages::{Messages, OutboundMessageBuilder, MessageFrom};
use tokio_core::reactor::Core;
use chrono::prelude::*;
use url::Url;

fn main() {
	/*
	let mut core = Core::new().unwrap_or_else(|err| {
		println!("Problem with core");
		process::exit(1);
	});
	let client = Client::new(&core.handle());

	let uri = "http://httpbin.org/ip".parse().unwrap_or_else(|err| {
		println!("Problem with uri");
		process::exit(1);
	});
	let work = client.get(uri).and_then(|res| {
		println!("Response: {}", res.status());

		res.body()
			.for_each(|chunk| io::stdout().write_all(&chunk).map_err(From::from))
	});
	core.run(work).unwrap_or_else(|err| {
		println!("Problem with running core");
		process::exit(1);
	});

	*/
	let mut core = Core::new().unwrap();
	let client = Client::new_from_env(&core.handle()).unwrap();
	let calls = Calls::new(&client);
    // Get a call by call sid
    /*
	let work = calls
		.get_call("CA166b2ee048446651bfccad9cdba48418")
		.map(|call| {
			println!(
				"Call sid is {}, parent call sid is {} and day of call is {:?}",
				call.sid,
				match call.parent_call_sid {
					None => "none found",
					Some(ref x) => x,
				},
                call.date_created.unwrap().weekday()
			);
			()
		});
	*/

    // Make outbound call
    /*
    let cb_url = Url::parse("https://handler.twilio.com/twiml/EHd118e2828f407106025378a044a91f26").unwrap();
    let fallback_url = Url::parse("https://www.example.com").unwrap();
	let outbound_call = OutboundCallBuilder::new("+15103674994", "+19493102155", &cb_url)
        .with_fallback_url(&fallback_url)
        .build();
	let work = calls.make_call(&outbound_call);
	*/

    // Get a calls list (paging)
    /*
    let work = calls.get_calls_with_page_size(5)
        .and_then(|page| {
            for call in page.items.iter() {
                println!("Call sid is {}", call.sid);
            }
            calls.get_next_page(&page)
        })
        .map(|opt_page| {
            if let Some(page) = opt_page {
                for call in page.items.iter() {
                    println!("Call sid is {}", call.sid);
                }
            }
            ()
        });
    */

    // Make a call, then redirect
    /*
	let queued_call = core.run(work).unwrap();

    thread::sleep(time::Duration::from_secs(15));

    let redirect_url = Url::parse("https://handler.twilio.com/twiml/EH09759ae9d76da9df9ce95c3a91fd3b73").unwrap();
    let work = calls.redirect_call(&queued_call.sid, &redirect_url, Some(CallbackMethod::Post));
    */

    let outbound_sms = OutboundMessageBuilder::new_sms(
        MessageFrom::From("+14088377998"),
        "+19493102155",
        "Hi from rust!"
    ).build();

    let messages = Messages::new(&client);
    let work =  messages.send_message(&outbound_sms);

    let msg = core.run(work).unwrap();
    println!("Sent message {}", msg.sid);


}
