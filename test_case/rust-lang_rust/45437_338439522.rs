rust
#![feature(conservative_impl_trait)] 
extern crate futures;

use futures::Future;
use futures::future::FutureResult;
use futures::future::ok;

fn future1(data: &i32) -> FutureResult<&i32, ()> {
    ok(data)
}

fn main() {
    future1(&1).wait().unwrap();
}
