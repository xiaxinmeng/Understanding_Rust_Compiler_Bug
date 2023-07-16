rust
#![feature(async_await, await_macro, futures_api)]

use futures::prelude::*;

async fn __receive<WantFn, Fut>(want: WantFn) -> ()
where
    Fut: Future<Output = ()>,
    WantFn: Fn(&Box<Send + 'static>) -> Fut,
{
    await!(futures::future::lazy(|_| ()));
}

pub fn basic_spawn_receive() {
    async { await!(__receive(|_| async { () })) };
}
