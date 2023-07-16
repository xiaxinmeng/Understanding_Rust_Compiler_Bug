rust
// crate a
use std::future::Future;

// NOTE: the pub(crate) here is critical
pub(crate) fn new() -> () {}

pub struct A;
impl A {
    fn call(&mut self, _: ()) -> impl Future<Output = ()> {
        async { new() }
    }
}
