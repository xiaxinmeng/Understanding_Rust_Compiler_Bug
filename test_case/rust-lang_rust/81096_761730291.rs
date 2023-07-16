rust
use std::future::Future;

async fn test(a: &str) {}

fn spawn<T>(task: T)
where
    T: Future + Send + 'static,
{
}

async fn main_async() {
    let owned = "Hello!".to_string();
    spawn(test(&owned));
}

fn main() {}
