rust
#![feature(unsized_locals)]

use std::ops::Index;

pub struct A;

impl Index<str> for A {
    type Output = ();
    fn index(&self, _: str) -> &Self::Output { panic!() }
}

fn main() {}
