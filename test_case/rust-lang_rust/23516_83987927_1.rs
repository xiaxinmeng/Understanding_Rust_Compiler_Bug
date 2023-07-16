 rust
// crate2
extern crate crate1;
use crate1::{Sugar, Sweet};

struct Cube;
impl Sugar for Box<Cube> { }
impl Sugar for Cube { }

fn foo<T:Sweet>() {
}

fn main() {
    foo::<Box<Cube>>();
}
