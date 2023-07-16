rust
use std::future::Future;
use std::pin::Pin;

async fn handle<F>(slf: &F)
where
    F: Fn(&()) -> Pin<Box<dyn Future<Output = ()>>>,
{
    (slf)(&()).await;
}
