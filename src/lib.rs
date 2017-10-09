extern crate futures;
extern crate hyper;
extern crate tokio_core;

use std::error::Error;
use std::process;
use std::thread;
use std::sync::mpsc;
use std::io::{self, Write};
use futures::{Future, Stream};
use tokio_core::reactor::{Core, Handle, Remote};
use hyper::client::{FutureResponse, HttpConnector};
use hyper::Body;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

pub struct Client {
    account_sid: String,
    auth_token: String,
    remote: Remote,
    client: hyper::Client<HttpConnector, Body>,
}

impl Client {
    pub fn new(account_sid: &str, auth_token: &str) -> Result<Client, io::Error> {
        let (tx, rx) = mpsc::channel();
        let (tx1, rx1) = mpsc::channel();
        thread::spawn(move || {
            let mut core = Core::new().unwrap();
            //let client = hyper::Client::new(&core.handle());
            tx.send(core.handle());
            tx1.send(core.remote());
            core.run(futures::empty()).unwrap();
        });
        let handle = rx.recv().unwrap();
        let remote = rx1.recv().unwrap();
        let client = hyper::Client::new(&handle);
        Ok(Client {
            account_sid: account_sid.to_string(),
            auth_token: auth_token.to_string(),
            remote: remote,
            client: client,
        })
    }

    pub fn send_request(&self) -> FutureResponse {
        let uri = "http://httpbin.org/ip".parse().unwrap_or_else(|err| {
            println!("Problem with uri");
            process::exit(1);
        });
        let get_task = self.client.get(uri);
        return get_task;
    }
}
