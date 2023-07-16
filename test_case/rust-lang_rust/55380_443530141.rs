Rust
#![feature(specialization)]

pub trait Foo {
    fn abc();
    fn def();
}

pub trait Marker {}

impl Marker for () {}

impl<T> Foo for T {
    default fn abc() {}
    default fn def() {}
}

impl<T: Marker> Foo for T {
    fn def() {
        Self::abc();
    }
}

fn main() {
    <()>::def();
    <i32>::def();
}
