 rust
impl<T> LinkedList<T> {
    pub fn reverse(&mut self) {
        let mut it = self.iter_mut();
        while let (Some(a), Some(b)) = (it.next(), it.next_back()) {
            std::mem::swap(a, b);
        }
    }
}
