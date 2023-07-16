rust
#[stable(feature = "rust1", since = "1.0.0")]
impl<A: Step> Iterator for ops::Range<A> where
    for<'a> &'a A: Add<&'a A, Output = A>
{
    type Item = A;

    #[inline]
    fn next(&mut self) -> Option<A> {
        if self.start < self.end {
            let mut n = self.start.add_one();
            // this line may not be optimized as well now for simple integer types
            // note this was the cause of unintentional recursion before ;)
            mem::swap(&mut n, &mut self.start); 
            Some(n)
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match Step::steps_between_by_one(&self.start, &self.end) {
            Some(hint) => (hint, Some(hint)),
            None => (0, None)
        }
    }
}
