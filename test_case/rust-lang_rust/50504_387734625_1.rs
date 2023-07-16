rust
// bar.rs
#![allow(unused)]

#[macro_use]
extern crate foo;

#[derive(Foo)] // OK?!
struct Dummy;

fn main() {}
