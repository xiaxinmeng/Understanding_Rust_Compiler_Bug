rust
#![feature(async_await)]

use std::future::Future;
use std::pin::Pin;

fn foo(n: usize) -> Pin<Box<dyn Future<Output = ()> + Send>> {
    Box::pin(async move {
        if n > 0 {
            foo(n - 1).await;
        }
    })
}

fn is_send<T: Send>(t: T) { drop(t); }

fn main() {
    is_send(foo(22));
}
