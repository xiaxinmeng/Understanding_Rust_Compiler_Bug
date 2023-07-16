rust
#![feature(self_struct_ctor)]
#![no_main]

pub struct Foo<T>(T);

impl<T> From<T> for Foo<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}
