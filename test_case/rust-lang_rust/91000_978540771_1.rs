rust
impl<T> [T] {
    fn raw_iter(&self) -> RawIter<T>;
    fn raw_iter_mut(&mut self) -> RawIterMut<T>;
}
