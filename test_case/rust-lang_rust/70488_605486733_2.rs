rust
use std::future::Future;

pub async fn foo(v: i32) -> i32 { v }

macro_rules! wrap {
    ($v:expr) => {
        async move { ($v).await }
    }
}

pub fn bar(v: i32) -> impl Future<Output = i32> {
    wrap!(wrap!(wrap!(wrap!(wrap!(wrap!(foo(v)))))))
}
