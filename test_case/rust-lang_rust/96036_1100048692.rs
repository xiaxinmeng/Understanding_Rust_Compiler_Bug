rust
#![crate_type = "lib"]

impl<T> Something<Whatever> for T { }    // blanket impl is duplicated on Whatever's page
pub struct Whatever { /* whatever */ }

pub trait Something<T> { }
