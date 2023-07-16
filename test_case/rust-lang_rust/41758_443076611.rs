rust
impl<T> Vec<T> {
    pub fn resize_with<F>(&mut self, new_len: usize, f: F)
    where
        F: FnMut() -> T;
}

impl<T: Clone> VecDeque<T> {
    pub fn resize_with(&mut self, new_len: usize, generator: impl FnMut() -> T);
}
