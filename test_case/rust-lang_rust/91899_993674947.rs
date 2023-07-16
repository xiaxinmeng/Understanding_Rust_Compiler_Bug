rust
pub trait ZeroCopyFrom {}
impl<T> ZeroCopyFrom for &'static T {}
