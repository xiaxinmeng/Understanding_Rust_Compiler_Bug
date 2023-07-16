 rust
// crate_a.rs
#![crate_type = "lib"]
pub use foo = abort;

extern { fn abort() -> !; }
