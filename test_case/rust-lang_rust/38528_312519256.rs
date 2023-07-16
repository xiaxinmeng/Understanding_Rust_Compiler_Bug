rust
extern crate futures;
use futures::{future, IntoFuture, Future};
fn main() {
    let t: std::result::Result<(), ()> = Ok(());
    let f = t
            .into_future()
            .and_then(|_| future::ok(()))
            .and_then(|_| future::ok(()))
            .and_then(|_| future::ok(()))
            .and_then(|_| future::ok(()))
            .and_then(|_| future::ok(()))
            .and_then(|_| future::ok(()))
            .and_then(|_| future::ok(()))
            .and_then(|_| future::ok(()))
            .and_then(|_| future::ok(()))
            .and_then(|_| future::ok(()))
            .and_then(|_| future::ok(()))
            .and_then(|_| future::ok(()))
            .and_then(|_| future::ok(()))
            .and_then(|_| future::ok(()))
            .and_then(|_| future::ok(()))
            .and_then(|_| future::ok(()))
            .and_then(|_| future::ok(()))
            .and_then(|_| future::ok(()));
    f.wait();
}
