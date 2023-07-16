rust
impl<'a, I: Iterator> Iterator for &'a mut I
    where I: ExactSizeIterator + DoubleEndedIterator
{
    fn rposition<P>(&mut self, predicate: P) -> Option<usize> where
        P: FnMut(Self::Item) -> bool,
        Self: ExactSizeIterator + DoubleEndedIterator
    {
        (**self).rposition(predicate)
    }
}
