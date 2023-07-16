 rust
#![feature(std_panic, recover)]
use std::panic;

fn main() {
    panic::recover(|| {
        panic!("hello");
    }).unwrap_err();
}
