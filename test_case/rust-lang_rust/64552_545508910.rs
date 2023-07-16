rust
use futures::future;
use futures::stream::{self, StreamExt};

struct Thing;

async fn genfoo(t: &[Thing]) {
    let _ = stream::iter(t)
        .map(future::ready)
        .buffer_unordered(10)
        .collect::<Vec<_>>()
        .await;
}

fn foo<'a>(t: &'a [Thing]) -> impl Send + 'a {
    genfoo(t)
}
