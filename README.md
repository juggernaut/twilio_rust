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
To send an SMS, you will also need a "from" number i.e a valid callerId in your Twilio accout, and the "to" number i.e the number you want to send the message to:
```bash
export FROM_NUMBER=<a valid callerId in your account>
export TO_NUMBER=<number you want to send message to>
```

```rust
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

let work = messages.send_message(&outbound_sms);
let sms = core.run(work).unwrap();
println!("Queued outbound SMS {}", sms.sid);
```
