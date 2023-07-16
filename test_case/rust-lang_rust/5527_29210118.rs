
impl<'self, A, T: Iterator<T>> Iterator<T> for &'self mut T {
    fn next(&mut self) -> Option<A> {
        (*self).next()
    }

    fn size_hint(&self) -> (uint, Option<uint>) {
        (*self).size_hint()
    }
}
