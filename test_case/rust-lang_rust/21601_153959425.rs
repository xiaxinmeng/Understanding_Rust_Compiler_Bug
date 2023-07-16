 rust
#![crate_type = "dylib"]
#![feature(rustc_private)]

extern crate syntax;

// One of the following item produces a `stack overflow` of rustc if
// one is commented. It's weird that it works with Deref.... The three cases:

// OK
// extern crate attribute;
// use std::ops::Deref;

// KO
extern crate attribute;
// use std::ops::Deref;

// OK
// // extern crate attribute;
// use std::ops::Deref;

pub use std::string::String;
pub use syntax::ast::Ident;

pub fn id_to_string(id: Ident) -> String
{
  id.to_string()
}
