# twilio_rust
A rust-lang client library for Twilio based on [hyper.rs](https://hyper.rs/). As such, network I/O is done asynchronously 
and all results are returned as futures.

# Getting started

Let's start with an example of sending an SMS (You can run this example with `cargo run --example send_message`).

You will need your Twilio credentials first:
```bash
export ACCOUNT_SID=<your account sid>
export AUTH_TOKEN=<your auth token>
```

```rust
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

let work = messages.send_message(&outbound_sms);
let sms = core.run(work).unwrap();
println!("Queued outbound SMS {}", sms.sid);
```
