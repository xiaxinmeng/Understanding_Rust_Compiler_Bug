rust
#![feature(associated_type_defaults)]

trait A {
    type B = Self::C;
    type C = Self::B;
}

impl A for () {}

fn main() {
    let x: <() as A>::B;
}
