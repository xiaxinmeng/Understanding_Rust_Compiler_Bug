 rust
// src/test/compile-fail/foo.rs
pub extern
  "invalid-abi" //~ ERROR: ...
fn foo() {}

fn main() {}
