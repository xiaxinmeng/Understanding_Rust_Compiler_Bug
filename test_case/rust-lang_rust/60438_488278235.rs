rust
// aux-build:arc_wake.rs
// edition:2018

#![allow(unused_variables)]
#![feature(async_await, await_macro)]

extern crate arc_wake;

use arc_wake::ArcWake;
use std::future::Future;
use std::sync::Arc;
use std::task::Context;

struct EmptyWaker;

impl ArcWake for EmptyWaker {
    fn wake(self: Arc<Self>) {}
}

async fn foobar_async(x: u32, (a, _, _c): (u32, u32, u32), _: u32, _y: u32) {
    assert_eq!(__arg1, (1, 2, 3)); // this doesn't compile error, but should
    assert_eq!(__arg2, 4);
}

fn main() {
    let empty = Arc::new(EmptyWaker);
    let waker = ArcWake::into_waker(empty);
    let mut cx = Context::from_waker(&waker);

    let mut fut = Box::pin(foobar_async(0, (1, 2, 3), 4, 0));
    let _ = fut.as_mut().poll(&mut cx);
}
