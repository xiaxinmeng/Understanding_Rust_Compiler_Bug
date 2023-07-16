 rust
pub struct EmptyIterator;
impl Iterator for EmptyIterator {
    type Item = ();
    fn next(&mut self) -> Option<<EmptyIterator as Iterator>::Item> {
        None
    }
}
