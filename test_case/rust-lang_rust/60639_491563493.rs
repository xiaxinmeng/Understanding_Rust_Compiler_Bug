rust
impl<T> *const [T] {
    fn get(self, idx: usize) -> Option<*const T>;
}
