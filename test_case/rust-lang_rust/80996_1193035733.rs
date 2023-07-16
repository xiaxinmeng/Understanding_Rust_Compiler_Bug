rust
impl RangeBounds<usize> for Region {
    fn start_bound(&self) -> Bound<&usize> {
        // TODO: Use `self.start.as_ref()` when upstream `std` stabilizes:
        // https://github.com/rust-lang/rust/issues/80996
        match self.start {
            Bound::Included(ref bound) => Bound::Included(bound),
            Bound::Excluded(ref bound) => Bound::Excluded(bound),
            Bound::Unbounded => Bound::Unbounded,
        }
    }

    fn end_bound(&self) -> Bound<&usize> {
        // TODO: Use `self.end.as_ref()` when upstream `std` stabilizes:
        // https://github.com/rust-lang/rust/issues/80996
        match self.end {
            Bound::Included(ref bound) => Bound::Included(bound),
            Bound::Excluded(ref bound) => Bound::Excluded(bound),
            Bound::Unbounded => Bound::Unbounded,
        }
    }
}
