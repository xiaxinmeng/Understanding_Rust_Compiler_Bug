rust
use futures::prelude::*;

fn foo() {
    fn assert_send<T: Send>(_: T) {}
    assert_send(bar());
}

async fn bar() -> Vec<i32> {
    let ints = [1, 2, 3];
    futures::stream::iter(&ints)
        .map(|i| async move { i.clone() })
        .buffer_unordered(2)
        .collect()
        .await
}
