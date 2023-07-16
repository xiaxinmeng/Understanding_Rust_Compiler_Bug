rust
// general block
impl<T: Clone, V: Borrow<[T]>> SliceConcatExt<T> for [V] {
    type Output = Vec<T>;
    default fn concat(&self) -> Vec<T> { ... }
    /* other methods */
}

// specializing block
impl<T: Copy, V: Borrow<[T]>> SliceConcatExt<T> for [V] {
    /* empty */
}
