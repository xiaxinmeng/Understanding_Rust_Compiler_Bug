rust
use std::future::Future;

use futures::{future::BoxFuture, stream, StreamExt};

async fn dummy() {}

fn inner() -> impl Future<Output = ()> {
    let some_vec = Vec::<()>::new();
    async move {
        let flat = stream::iter(&some_vec)
            .map(|_| stream::empty::<()>())
            .flatten();

        std::mem::drop(flat);
        dummy().await;
    }
}

fn outer() -> BoxFuture<'static, ()> {
    Box::pin(async move { inner().await })
}
