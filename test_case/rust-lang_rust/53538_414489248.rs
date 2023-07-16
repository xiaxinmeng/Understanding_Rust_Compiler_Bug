rust
impl<T> [T] {
    fn insert_shift(&mut self, i: usize, value: T) {
        self[i..].rotate_right(1);
        self[i] = value;
    }
}
