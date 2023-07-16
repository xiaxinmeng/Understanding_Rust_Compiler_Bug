rust
// A 2015-edition crate using the proc-macro "pmf".
#![warn(absolute_paths_not_starting_with_crate)]
extern crate pmf;

pub trait Foo {}

#[derive(pmf::Foo)]
#[helper = "::Bar"]
pub struct S;

pub struct Bar;
