 rust
// lib.rs
include!("foo.rs");

fn main() {}

// foo.rs
fn foo() {}

#[cfg(nope)]
mod bar {
    include!("bar.rs");
}

// bar.rs
fn foo() {}
