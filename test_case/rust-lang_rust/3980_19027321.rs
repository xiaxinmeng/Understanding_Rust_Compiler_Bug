
impl<T: Copy + Add<T,T>> Add<Option<T>, Option<T>> for Option<T> {
    #[inline(always)]
    fn add(&self, other: &Option<T>) -> Option<T> {
        match (*self, *other) {
            (None, None) => None,
            (_, None) => *self,
            (None, _) => *other,
            (Some(ref lhs), Some(ref rhs)) => Some(*lhs + *rhs)
        }
    }
}
