rust
use std::{future, marker::PhantomData};

async fn foo() {
    future::ready(PhantomData::<*mut ()>).await;
}

fn assert_send_sync(_: impl Send, _: impl Sync) {}

fn main() {
    assert_send_sync(foo(), foo());
}
