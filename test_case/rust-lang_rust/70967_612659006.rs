rust
impl<T> Vec<T> {
    /// Returns the vector's spare capacity as a mutable slice of maybe-uninitialized values.
    ///
    /// The length of the slice will be `self.capacity() - self.len()`.
    pub fn spare_capacity_mut(&mut self) -> &mut [MaybeUninit<T>] { ... }
}
