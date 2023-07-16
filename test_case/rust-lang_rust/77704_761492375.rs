rust
/// Performs bounds-checking of this range.
///
/// The returned [`Range`] is safe to pass to [`slice::get_unchecked`] and
/// [`slice::get_unchecked_mut`] for slices of the given length.
#[unstable(feature = "range_bounds_assert_len", issue = "76393")]
fn assert_len(self, len: usize) -> Range<usize>
where
    Self: RangeBounds<usize>;
