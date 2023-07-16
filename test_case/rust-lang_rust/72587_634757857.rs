rust
#![warn(unused_lifetimes)]
fn unused_lifetime<'a>() {}
fn main() {unused_lifetime();}
