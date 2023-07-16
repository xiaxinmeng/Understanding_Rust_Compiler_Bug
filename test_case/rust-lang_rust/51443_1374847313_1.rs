rust
pub trait Trait {
    // want to call this on non-Sync impls
    fn synchronous(&self) {}

    // as long as we're using Send futures (e.g. tokio) this only makes
    // sense to call on Sync impls, hence the where-clause
    fn asynchronous(&self) -> Pin<Box<dyn Future<Output = ()> + Send + '_>>
    where
        Self: Sync,
    {
        Box::pin(async move { ... })
    }
}
