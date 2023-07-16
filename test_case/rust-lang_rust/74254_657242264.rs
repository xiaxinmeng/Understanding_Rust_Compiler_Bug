rust
#![feature(const_generics)]

struct Foo<const N: usize>([u8; N]);

impl<const N: usize> Foo<N> {
    fn new() -> Self {
        Foo(Default::default())
    }
}

fn main() {}
