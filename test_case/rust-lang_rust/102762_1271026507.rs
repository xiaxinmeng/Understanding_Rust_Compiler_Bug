rust
use std::future::Future;
use std::pin::Pin;

pub trait Fetcher: Send + Sync {
    fn get<'a>(self: &'a Box<Self>) -> Pin<Box<dyn Future<Output = Vec<u8>> + 'a>>
    where
        Self: Sync,
    {
        todo!()
    }
}

fn fetcher() -> Box<dyn Fetcher> {
    todo!()
}

pub fn foo() {
    let fetcher = fetcher();
    let _ = fetcher.get();
}
