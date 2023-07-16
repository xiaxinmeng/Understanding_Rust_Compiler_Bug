 rust
let (tx, rx) = channel();
let timer = Timer::new().unwrap();
let timeout = timer.oneshot(10000);
select!(
    val = rx.recv() => println!("Received {}", val),
    () = timeout.recv() => println!("timed out, no message received in 10 seconds"),
)
