rs
pub trait A {
    fn test();
}

pub struct Foo;

impl A for Foo {
    fn test() {}
}
