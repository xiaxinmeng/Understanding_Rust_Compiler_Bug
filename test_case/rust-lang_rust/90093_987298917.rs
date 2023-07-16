rust
pub trait InfiniteIterator: Iterator {
    fn next_infinite(&mut self) -> Self::Item {
        match self.next() {
            Some(x) => x,
            None => infinite_iterator_was_finite(),
        }
    }
}

#[cold]
fn infinite_iterator_was_finite() -> ! {
    unreachable!()
}
