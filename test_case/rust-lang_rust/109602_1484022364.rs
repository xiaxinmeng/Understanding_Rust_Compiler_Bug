rust
#![crate_type = "lib"]
extern crate f;

pub use inner::f;

pub use f as g;

g!{}

fn main() {}
