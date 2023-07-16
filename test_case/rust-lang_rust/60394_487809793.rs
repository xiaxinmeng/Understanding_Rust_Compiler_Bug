rust
use futures::{future, Future};

fn f() -> impl Future<Item = (), Error = ()> {
    let _u = unimplemented!();
    future::ok(_u)
}

fn main() {
    tokio::run(f());
}
