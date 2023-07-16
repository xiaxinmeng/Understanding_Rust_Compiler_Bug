rust
impl<T> RangeInclusiveEquality for T {
    #[inline]
    default fn canonicalized_is_empty(range: &RangeInclusive<Self>) -> bool {
        false // the exact value doesn't matter when range is not an iterator
    }
}
