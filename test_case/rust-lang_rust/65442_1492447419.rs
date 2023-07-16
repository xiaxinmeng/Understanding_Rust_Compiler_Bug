rust
#![feature(type_alias_impl_trait)]

use std::future::Future;
use tokio::sync::oneshot::Sender;

type Deferred = impl Future<Output = ()> + 'static;

fn sends_future_kludge<F: Future<Output = ()> + 'static>(f: F) -> Deferred {
    async move { f.await }
}

fn sends_future(tx: Sender<Deferred>) {
    let _ = tx.send(sends_future_kludge(async move {}));
}
