 rust
#![feature(staged_api)]
#![crate_type = "lib"]
#![staged_api]
#![stable(feature = "mycrate", since = "1.0.0")]

#[unstable(feature = "mycrate_internals")]
mod bar {}
#[stable(feature = "mycrate", since = "1.0.0")]
pub fn bar() {}
