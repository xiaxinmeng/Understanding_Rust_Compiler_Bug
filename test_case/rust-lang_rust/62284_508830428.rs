rust
#![feature(async_await)]

use {
    std::{
        future::Future,
        pin::Pin,
    },
};

type BoxFuture = Pin<Box<dyn Future<Output = ()> + Send>>; // adding Send causes a cycle error

fn foo() -> impl Future<Output = BoxFuture> + Send {
    async {
        Box::pin(bar()) as BoxFuture
    }
}

async fn bar() {
    let _ = foo().await;
}

fn main() { }
