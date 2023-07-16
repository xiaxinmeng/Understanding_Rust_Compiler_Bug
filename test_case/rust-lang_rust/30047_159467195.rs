 rust
// slowtest.rs
use std::thread;

#[test]
fn foo() {
    thread::sleep_ms(1000);
}

#[test]
fn bar() {
    thread::sleep_ms(500);
}
