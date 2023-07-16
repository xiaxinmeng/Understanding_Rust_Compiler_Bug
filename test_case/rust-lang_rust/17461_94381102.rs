
use std::marker::PhantomData;

#[derive(Hash, Eq, PartialEq)]
struct A<'a> {
    x: u64,
    marker: PhantomData<&'a ()>,
}

fn main() { }
