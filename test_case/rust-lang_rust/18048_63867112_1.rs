 rust
// foo.rs
#![crate_type="lib"]
#![feature(associated_types)]

extern crate bar;

pub use bar::Bar;
