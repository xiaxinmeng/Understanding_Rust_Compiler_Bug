rust
#![feature(register_tool)]
#![no_implicit_prelude]
#![register_tool(a)]

#[a::skip]
fn main() {}
