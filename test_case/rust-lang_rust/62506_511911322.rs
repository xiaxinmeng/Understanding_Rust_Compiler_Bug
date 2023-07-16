
#![feature(async_await)]
use std::future::Future;

pub trait T {
    type Future: Future<Output = String>;
    fn bar() -> Self::Future;
}
pub async fn foo<S>() where S: T {
    S::bar().await;
    S::bar().await;
}
