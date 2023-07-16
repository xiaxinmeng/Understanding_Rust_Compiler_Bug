
impl<T> ~[T] {
    fn remove(&mut self, index: uint) -> T {
        assert!(index < self.len());
        if (index != self.len() - 1) {
            self.swap(index, self.len() - 1);
        }
        self.pop();
    }
}
