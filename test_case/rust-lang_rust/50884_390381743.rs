rust
pub struct IteratedCheck<I> {
    iter: I,
    iterated: bool,
}
impl<I: Iterator> Iterator for I {
    type Item = <I as Iterator>::Item;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iterated = true;
        self.iter.next()
    }
    // forward other methods too
}
impl<T, U> Sum<U> for Option<T>
where
    T: Sum<U>,
{
    #[inline]
    fn sum<I: Iterator<Item = U>>(iter: I) -> Option<T> {
        let mut adapter = IteratedCheck { iter, iterated: false };
        Some(adapter.by_ref().sum()).filter(adapter.iterated)
    }
}
// and product too
