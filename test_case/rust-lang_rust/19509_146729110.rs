 rust
use std::ops::Deref;

pub struct Foo {
    baz: Bar,
}
pub struct Bar;

impl Foo {
    pub fn foo_method(&self) {
        /* do something, doesn't really matter */
    }
}

impl Bar {
    pub fn bar_method(&self) {

    }
}

impl Deref for Foo {
    type Target = Bar;

    fn deref<'a>(&'a self) -> &'a Self::Target {
        &self.baz
    }
}

impl Deref for Bar {
    type Target = Foo;

    fn deref<'a>(&'a self) -> &'a Self::Target {
        panic!()
    }
}

fn main() {
    let foo = Foo {
        baz:    Bar,
    };
    let bar = Bar;

    // should work
    foo.bar_method();
    // should compile but panic on runtime
    bar.foo_method();
}
