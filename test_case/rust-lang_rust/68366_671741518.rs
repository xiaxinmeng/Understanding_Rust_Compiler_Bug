rust
#![feature(const_generics)]

struct Collatz<T>(T);

trait Foo {
    type Assoc;
}

impl<T: Foo> Collatz<T::Assoc> {}

fn main() {}
