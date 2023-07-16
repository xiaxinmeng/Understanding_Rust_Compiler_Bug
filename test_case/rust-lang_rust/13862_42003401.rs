
let (tx, rx) = channel();
let mut rx = TimeoutReceiver::new(rx, 10000);
match rx.recv() {
    Some(val) => println!("Received {}", val),
    None => println!("timed out, no message received in 10 seconds"),
}
