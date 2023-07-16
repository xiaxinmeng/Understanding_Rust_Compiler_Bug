rust
fn wrap_future(x: impl core::future::Future) -> impl core::future::Future {
    async { x.await }
}
