rust
impl<I> Iterator for Rev<I> where I: DoubleEndedIterator {
    ...
    #[inline]
    fn last(mut self) -> Option<Self::Item> {
        self.next_back()
    }
    ...
}
