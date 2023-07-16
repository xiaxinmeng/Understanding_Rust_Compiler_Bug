rust
use std::future::Future;

async fn foo<'a, F, R>(f: F) -> bool
where
    F: Fn(&'a u8) -> R, // <- note explicit lifetime here
    R: Future<Output = bool>,
{
    todo!()
}

async fn bar() {
    foo(fut).await;
}

async fn fut(param: &u8) -> bool {
    todo!()
}
