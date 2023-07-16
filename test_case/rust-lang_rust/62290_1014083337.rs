rust
fn takes_async_closure<F: Future<Output = ()>>(_:  impl Fn() -> F)
