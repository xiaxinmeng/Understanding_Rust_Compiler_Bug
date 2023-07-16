
#[inline]
fn is_sorted_by<F>(mut self, compare: F) -> bool
where
    Self: Sized,
    F: FnMut(&Self::Item, &Self::Item) -> Option<Ordering>
{
    self.make_slice().is_sorted_by(compare)
}
