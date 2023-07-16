rust
use std::future::Future;
use std::pin::Pin;

fn boxed<'a, F, T>(x: F) -> Pin<Box<dyn Future<Output = T> + Send + 'a>>
where
    F: Future<Output = T> + Sized + Send + 'a,
{
    Box::pin(x)
}

async fn foo() {
    boxed(async {
        foo().await;
    });
}

fn main() {
    let _ = foo();
}
