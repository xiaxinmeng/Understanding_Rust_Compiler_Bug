rust
#![feature(const_generics)]

pub struct Foo<T, const N: usize>([T; N]);

impl<T, const N: usize> Foo<T, {N}> {
    pub fn new() -> Self {
        unimplemented!()
    }
}

fn main() {
    let _: Foo<u32, 0> = Foo::new();
}
