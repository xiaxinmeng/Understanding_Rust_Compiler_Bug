 rust
// lib.rs
include!("foo.rs");

fn main() {}

// foo.rs
fn foo() {}

#[cfg(nope)]
include!("bar.rs");

// bar.rs
fn foo() {}
