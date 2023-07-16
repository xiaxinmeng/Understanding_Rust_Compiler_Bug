rust
// This is to allow coercion from `Ref<T>` to `Ref<U>` if `T` can be converted to the
// dynamically-sized type (DST) `U`.
impl<T: ?Sized + Unsize<U>, U: ?Sized> core::ops::CoerceUnsized<Ref<U>> for Ref<T> {}
