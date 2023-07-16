rust
use futures::{
    future::ready,
    stream::{empty, iter},
    Stream, StreamExt,
};
use std::pin::Pin;

fn is_send<T: Send>(_: T) {}

pub fn test() {
    let _ = is_send(make_future_2());
}

// Same as make_future, just async
async fn make_future_2() {
    iter(Some({
        let s = empty::<()>();
        Box::pin(s) as Pin<Box<dyn Stream<Item = ()> + Send + 'static>>
    }))
    .map(|_| empty::<()>())
    .flatten()
    .for_each(|_| ready(()))
    .await
}

fn main() {}
