rust
use futures::{future::BoxFuture, stream, StreamExt};

async fn inner() {
    stream::iter(&Vec::<()>::new())
        .map(|_| stream::empty::<()>())
        .flatten()
        .collect::<()>()
        .await;
}

fn outer() -> BoxFuture<'static, ()> {
// error: higher-ranked lifetime error
    Box::pin(async move { inner().await })
//  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
// = note: could not prove `Pin<Box<impl futures::Future<Output = ()>>>: CoerceUnsized<Pin<Box<
//         (dyn futures::Future<Output = ()> + std::marker::Send + 'a)>>>`
}
