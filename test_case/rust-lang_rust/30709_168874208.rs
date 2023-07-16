 rust
///////
// A.rs
///////

#![crate_type = "lib"]

pub trait A { }

///////
// B.rs
///////

#![crate_type = "lib"]

extern crate A;

pub struct X;

impl A::A for X { }

#[link_name = "xyz"]
extern { }

///////
// C_wrong.rs
///////

#![crate_type = "lib"]

extern crate A;
extern crate B;

///////
// C_fixed.rs
///////

#![crate_type = "lib"]

extern crate A;
#[cfg(b)] extern crate B;
