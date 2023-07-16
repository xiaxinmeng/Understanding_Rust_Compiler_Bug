 rust
impl<T: Clone + Copy> Clone for S<T> {
    fn clone(&self) -> S<T> {
        *self
    }
}
