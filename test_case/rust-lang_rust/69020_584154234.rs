rust
use std::i32;

pub trait Foo {
    const N: i32;
}

impl<T: Foo> Foo for Vec<T> {
    const N: i32 = -i32::MIN + T::N;
}
