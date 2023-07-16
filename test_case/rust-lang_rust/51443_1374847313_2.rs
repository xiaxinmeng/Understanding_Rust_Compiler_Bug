rust
pub trait Trait: Sync {
    fn synchronous(&self) {}
    fn asynchronous(&self) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        Box::pin(async move { ... })
    }
}
