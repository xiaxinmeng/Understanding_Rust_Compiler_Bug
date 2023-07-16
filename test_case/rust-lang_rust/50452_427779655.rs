rust
#![feature(specialization)]

pub trait Foo {
    fn foo();
}

impl Foo for i32 {}
impl Foo for i64 {
    // fn foo() {}
}
impl<T> Foo for T {
    fn foo() {}
}
