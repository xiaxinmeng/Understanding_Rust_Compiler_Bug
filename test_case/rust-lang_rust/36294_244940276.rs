 rust
use std::sync::{Arc, Mutex};
use std::any::Any;

fn main() {
    loop {
        drop(foo());
    }
}

fn foo() -> Box<Mutex<Any>> {
    Box::new(Mutex::new(0u32)) as Box<Mutex<Any>>
}
