rust
// foo/src/lib.rs
#![feature(conservative_impl_trait)]
extern crate futures;
use futures::{Future, future};

pub struct Foo;
impl Foo {
    fn id(self) -> Self {
        self
    }

    pub fn new() -> impl Future<Item = Self, Error = ()> + 'static {
        future::lazy(|| future::ok(Foo.id()))
    }
}
