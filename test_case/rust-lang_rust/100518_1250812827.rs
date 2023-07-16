rust
struct DefinitelyNotFused<I>(I);
impl<I: Iterator> Iterator for DefinitelyNotFused<I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> { self.0.next() }
}
