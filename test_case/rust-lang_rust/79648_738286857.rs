rust
use futures::stream;
use futures::stream::StreamExt;
use std::future::{self, Future};

fn get_foo<'a>() -> impl Future<Output = Vec<()>> + 'a {
    async {
        let results = vec![()];
        let foo: Vec<()> = stream::iter(results)
            .filter_map(|_| async move { Some(()) })
            .filter(|m| future::ready(true))
            .collect()
            .await;
        foo
    }
}
fn demand_is_send<T>(t: T)
    where T: Send
{}
fn main() {
    demand_is_send(get_foo())
}
