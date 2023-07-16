rust
#![feature(proc_macro, conservative_impl_trait, generators, nll)]

extern crate futures_await as futures;
extern crate tokio_core;
extern crate tokio_retry;

use futures::prelude::*;
use tokio_core::reactor::Handle;
use tokio_retry::Retry;
use tokio_retry::strategy::ExponentialBackoff;

#[async]
fn something(handle: Handle) -> Result<(), ()> {
    let retry_strategy = ExponentialBackoff::from_millis(10).take(3);

    let retry_future = Retry::spawn(handle, retry_strategy, move || {
        let future_of_1 = futures::future::ok::<(), ()>(());
        future_of_1
    }).map_err(|_| ());

    await!(retry_future)
}

fn main() {}
