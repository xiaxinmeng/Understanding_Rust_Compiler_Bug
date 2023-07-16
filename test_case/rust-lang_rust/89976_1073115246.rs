rust
use futures::stream::{self, StreamExt};
use std::future::{ready, Ready};

/// Enforce the passed closure is generic over its lifetime
fn type_hint<F>(f: F) -> F
where
    F: for<'a> FnMut(&'a String) -> Ready<()>,
{
    f
}

// Closure correctly gets correctly inferred
async fn working1(input: &[String]) {
    let mut foo = stream::iter(input)
        .map(|_| ready(()))
        .buffer_unordered(5)
        .boxed();
    foo.next().await;
}

// removing the .boxed() makes it fail
async fn broken1(input: &[String]) {
    let mut foo = stream::iter(input)
        .map(|_| ready(()))
        .buffer_unordered(5);

    foo.next().await;
}

// adding a type hint makes it work again
async fn working2(input: &[String]) {
    let mut foo = stream::iter(input)
        .map(type_hint(|_| ready(())))
        .buffer_unordered(5);

    foo.next().await;
}

async fn spawner() {
    tokio::spawn(working1(&[]));
    tokio::spawn(broken1(&[]));
    tokio::spawn(working2(&[]));
    // Directly awaiting works fine though
    broken1(&[]).await;
}
