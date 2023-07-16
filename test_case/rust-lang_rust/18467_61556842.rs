 rust
impl<'a, T: Ord> Ord for &'a T {
    #[inline]
    fn cmp(&self, other: & &'a T) -> Ordering { (**self).cmp(*other) }
}
