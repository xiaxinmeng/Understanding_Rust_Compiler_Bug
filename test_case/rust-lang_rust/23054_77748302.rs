
#[stable(feature = "rust1", since = "1.0.0")]
pub use intrinsics::copy_memory as copy;

#[unstable(feature = "core")]
#[deprecated(since = "1.0.0", reason = "renamed to `copy`")]
pub fn copy_memory<T>(dst: *mut T, src: *const T, count: usize);
