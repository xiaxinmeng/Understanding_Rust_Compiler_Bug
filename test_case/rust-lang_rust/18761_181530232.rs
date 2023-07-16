
#![feature(default_type_params)]

pub trait Foo<A, B = ()> {}

pub struct Wrap<F> {
    foo: F,
}

impl<A: Send, F: Foo<A>> Wrap<F> {
    fn stuff(&self) {
    }
}

struct MyFoo;

impl Foo<usize, usize> for MyFoo {}

pub fn main() {
    let w = Wrap { foo: MyFoo };
    w.stuff();
}
