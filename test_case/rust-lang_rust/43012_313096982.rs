rust
struct MaybeReverse<I> {
    iter: I,
    reverse: bool,
}

impl<I: DoubleEndedIterator> Iterator for MaybeReverse<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        if self.reverse {
            self.iter.next_back()
        } else {
            self.iter.next()
        }
    }
}
