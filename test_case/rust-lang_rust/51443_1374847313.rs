rust
#[async_trait]
pub trait Trait {
    fn synchronous(&self) {}
    async fn asynchronous(&self) {}
}
