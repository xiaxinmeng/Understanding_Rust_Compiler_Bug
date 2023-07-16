 rust
use std::vec::CowVec;

#[deriving(Clone)]
struct Foo<'a, T: 'a + Clone> {
    bar: CowVec<'a, T>,
}

fn main() {}
