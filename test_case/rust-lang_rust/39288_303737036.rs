rust
impl<T> Option<T> {
    fn get_or_insert(&mut self, v: T) -> &mut T;
    fn get_or_insert_with<F: FnOnce() -> T>(&mut self, f: F) -> &mut T;
}
