 rust
#![feature(std_panic, panic_handler)]
use std::panic;

fn main() {
    let orig_handler = panic::take_handler();
    panic::set_handler(|_| ());
    // <an actual thing>
    panic::set_handler(move |info| (*orig_handler)(info));
}
