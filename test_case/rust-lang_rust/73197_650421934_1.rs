rust
// Bound constraints suffer the same problem, suppose an imaginary std Number trait that has `min()` and `max()`
impl<T: Number> Default for Range<T> { fn default() -> Self { T::min()..T::max() } }

// As a consequence, it also won't be possible to have a generic implementation for Range<T>
