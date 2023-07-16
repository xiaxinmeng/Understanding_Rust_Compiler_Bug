
impl<A, B> DoubleEndedIterator for Chain<A, B> where
    A: DoubleEndedIterator,
    B: DoubleEndedIterator<Item=A::Item>,
{
    #[inline]
    fn next_back(&mut self) -> Option<A::Item> {
        match self.b.next_back() {
            Some(x) => Some(x),
            None => self.a.next_back()
        }
    }
}
