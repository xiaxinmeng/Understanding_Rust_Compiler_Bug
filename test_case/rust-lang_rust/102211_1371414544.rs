rust
#![allow(incomplete_features)]
#![feature(return_position_impl_trait_in_trait)]

use anyhow::Result;
use futures::{pin_mut, Future, Stream, StreamExt};
use tokio::task;

#[tokio::main]
async fn main() -> Result<()> {
    Ok(())
}

async fn run<T>(mut foo: T) -> Result<()>
where
    T: Foo + Send + 'static,
{
    task::spawn(async move {
        let bar = assert_send(foo.bar());
        let bar = bar.await; // `impl Stream<Item = i32> + Send`
        let mut bar = stream_assert_send(Box::pin(bar)); // still `impl Stream<Item = i32> + Send`

        // higher-ranked lifetime error
        // bar.next().await;

        let next = assert_send(bar.next());
        // Also higher-ranked lifetime error
        next.await;
    });

    Ok(())
}

fn assert_send<'a, T>(
    fut: impl std::future::Future<Output = T> + Send + 'a,
) -> impl std::future::Future<Output = T> + Send + 'a {
    fut
}

fn stream_assert_send<'a, T>(
    stream: impl futures::Stream<Item = T> + Send + 'a,
) -> impl futures::Stream<Item = T> + Send + 'a {
    stream
}

trait Foo {
    fn bar<'a>(&'a mut self) -> impl Future<Output = impl Stream<Item = i32> + Send + 'a> + Send;
}
