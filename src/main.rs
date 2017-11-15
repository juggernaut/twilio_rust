extern crate chrono;
extern crate futures;
extern crate hyper;
extern crate tokio_core;

extern crate twilio_rust;
extern crate url;

use std::env;
use std::process;
use std::io::{self, Write};
use futures::{Future, Stream};
use twilio_rust::{Client, Page};
use twilio_rust::calls::{Calls, OutboundCall, OutboundCallBuilder};
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
    let cb_url = Url::parse("https://handler.twilio.com/twiml/EHd118e2828f407106025378a044a91f26").unwrap();
    let fallback_url = Url::parse("https://www.example.com").unwrap();
	let outbound_call = OutboundCallBuilder::new("+15103674994", "+19493102155", &cb_url)
        .with_fallback_url(&fallback_url)
        .build();
	let work = calls.make_call(&outbound_call);
	*/
    let work = calls.get_calls_with_page_size(5)
        .map(|page| {
            for call in page.items.iter() {
                println!("Call sid is {}", call.sid);
            }
            ()
        });
	core.run(work).unwrap();
}
