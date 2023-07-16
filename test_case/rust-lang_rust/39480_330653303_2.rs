rust
impl<'a, I: DoubleEndedIterator> DoubleEndedIterator for &'a mut I {
    fn rfind<P>(&mut self, predicate: P) -> Option<Self::Item>
        where P: FnMut(&Self::Item) -> bool,
    {
        (**self).rfind(predicate)
    }
}
