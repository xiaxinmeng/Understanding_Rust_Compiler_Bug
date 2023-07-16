
#![crate_type="lib"]

use std::sync::Arc;

pub struct Wrapper {
    x: usize,
    y: usize,
    t: Arc<Foo>,
}

pub trait Foo {
    fn foo(&self);
}

pub fn test(foo: Wrapper) {
    for _ in 0..200 {
        foo.t.foo();
    }
}
