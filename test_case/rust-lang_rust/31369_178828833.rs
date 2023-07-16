 rust
// lib.rs
include("foo.rs");

// foo.rs
fn foo() {}

#[cfg(feature = "never")]
fn foo() {}
