rust
impl<'a, I: Iterator> Iterator for &'a mut I {
    fn find<P>(&mut self, predicate: P) -> Option<Self::Item>
        where P: FnMut(&Self::Item) -> bool,
    {
        (**self).find(predicate)
    }
}
