rust
/// This is usually the same as `size_of::<T>()`. However, when `T` *has* no
/// statically-known size, e.g., a slice [`[T]`][slice] or a [trait object],
/// then `size_of_val` can be used to get the dynamically-known size.
