rust
pub trait CaptureLifetime<'a> {}
impl<T: ?Sized> CaptureLifetime<'_> for T {}
