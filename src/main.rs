extern crate chrono;
extern crate futures;
extern crate hyper;
extern crate tokio_core;

extern crate twilio_rust;

use std::env;
use std::process;
use std::io::{self, Write};
use futures::{Future, Stream};
use twilio_rust::Client;
use twilio_rust::calls::Calls;
use tokio_core::reactor::Core;
use chrono::prelude::*;

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
				call.date_created.weekday()
			);
			()
		});
	core.run(work).unwrap();
	println!("This was executed!");
}
