rust
trait MyIterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

impl<I: MyIterator> MyIterator for &mut I { // No 'a
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        (**self).next()
    }
}
