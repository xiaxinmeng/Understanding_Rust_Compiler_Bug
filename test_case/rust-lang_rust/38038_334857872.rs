rust
impl<I> DoubleEndedIterator for Chain2<I> where I: DoubleEndedIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(ref mut b) = self.b {
            if let elt @ Some(_) = b.next() {
                return elt;
            }
        }
        self.b.take();
        self.a.next()
    }
}
