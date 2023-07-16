 rust
struct EmptyIterator;
impl Iterator for EmptyIterator {
    type Item = ();
    fn next(&mut self) -> Option<Iterator::Item> {
        None
    }
}
