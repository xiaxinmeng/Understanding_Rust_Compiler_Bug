 rust
use std::thread;
use std::time::Duration;

#[test]
fn ok() {
    thread::sleep(Duration::from_secs(70));
}

#[test]
fn panic() {
    thread::sleep(Duration::from_secs(70));
    panic!();
}
