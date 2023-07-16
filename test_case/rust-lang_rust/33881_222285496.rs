 rust
impl<'a> PeekableIterator for Chars<'a> {
    #[inline]
    fn peek(&self) -> Option<Self::Item> {
        self.clone().next()
    } 
}
