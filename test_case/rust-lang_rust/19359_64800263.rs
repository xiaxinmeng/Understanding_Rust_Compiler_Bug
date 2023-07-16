 rust
use std::vec::CowVec;

#[deriving(Clone)]
struct Foo<'a, T: 'a> where T: Clone {
    bar: CowVec<'a, T>,
}

fn main() {}
