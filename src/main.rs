extern crate futures;
extern crate hyper;
extern crate tokio_core;

extern crate twilio_rust;

use std::process;
use std::io::{self, Write};
use futures::{Future, Stream};
use twilio_rust::Client;

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

	let client = Client::new("ac", "auth").unwrap();
	client
		.send_request()
		.and_then(|res| {
			println!("Response: {}", res.status());
			res.body()
				.for_each(|chunk| io::stdout().write_all(&chunk).map_err(From::from))
		})
		.wait();
	println!("Ok this executed!");
}
