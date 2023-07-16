rust
impl<'a, R, T: ?Sized + 'a> RangeBounds<T> for R where R: RangeBounds<&'a T> {
    // …
}
