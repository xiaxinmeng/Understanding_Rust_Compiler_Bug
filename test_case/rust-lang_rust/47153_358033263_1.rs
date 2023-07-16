rust
#![feature(nll)]

struct Foo<T> { x: T }

impl<T> Foo<T> {
    const B: bool = true;
}

fn main() { }
