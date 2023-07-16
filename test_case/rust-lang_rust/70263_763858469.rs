rust
use std::future::Future;

struct A {}
struct B {}

async fn f<'a, G, Fut>(arg: &'a A, g: G)
where
    for<'b> G: Fn(&'a A, &'b B) -> Fut,
    Fut: Future<Output = String> + 'a,
{
    let b = B {};
    let ans = g(arg, &b).await;
}

fn main() {
    f(&A {}, async_func);
}

async fn async_func(a: &A, b: &B) -> String {
    "test".to_owned()
}
