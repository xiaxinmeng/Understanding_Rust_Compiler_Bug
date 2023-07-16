 rust
use std::ops::Deref;

struct Foo<T> {
    xs: [T; 0]
}

impl<T> Deref for Foo<T> {
    type Target = [T];
    fn deref(&self) -> <Self as Deref>::WhyDoesThisCompile {
        123
    }
}

fn main() {
}
