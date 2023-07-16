 rust
pub struct Internal<I> {
    iter: I
}

impl Internal<()> {
    fn new<It: IntoIterator>(iterable: It) -> Internal<It::IntoIter> {
        let iter = iterable.into_iter();
        Internal { iter: iter }
    }
}
