rust
/// An iterator able to step backwards.
pub trait BidirectionalIterator: Iterator {
    /// Steps backwards in the iterator and returns the previous value.
    ///
    /// Returns [`None`] when there is no previous item left to return.
    fn prev(&mut self) -> Option<Self::Item>;
}
