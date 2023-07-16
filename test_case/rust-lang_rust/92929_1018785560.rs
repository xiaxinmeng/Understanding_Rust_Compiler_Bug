rust
use std::pin::Pin;
use std::future::Future;

fn main() {
    let f1: Pin<Box<_>> = Box::new(async { true }).into();
    let _f1: Pin<Box<dyn Future<Output = bool> + Send + 'static>> = f1;
}
