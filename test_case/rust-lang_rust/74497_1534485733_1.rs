rust 
use std::future::Future;

pub async fn bar() {
    foo(|x| baz(x)).await;
}

pub async fn baz(x: &u8) -> bool {
    if *x == 1 {
        false
    } else {
        true
    }
}

pub async fn foo<'a, F, T>(f: F) -> bool
where
    F: Fn(&'a u8) -> T,
    T: Future<Output = bool>,
{
    f(&32).await
}

fn main() {}
