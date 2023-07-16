 rust
// foo.rs
use inner::Trait;

mod inner {
    pub struct Foo;
    pub trait Trait {
        fn f(&self);
    }

    impl Trait for Foo {
        fn f(&self) { }
    }
}

pub trait Outer {
    fn foo<T: Trait>(&self, t: T) { t.f(); }
}

impl Outer for int {}

pub fn foo<T: Outer>(t: T) {
    t.foo(inner::Foo);
}
