 rust
#![allow(unused_variable)]
use std::any::Any;

fn main() {
    let foo: u64 = 1;
    let raw_any_foo = &foo as *const Any;
}
