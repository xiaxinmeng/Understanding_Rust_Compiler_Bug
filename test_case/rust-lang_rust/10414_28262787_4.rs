
impl<T> Iterator<T> for Range<T> {
    fn next() -> Option<T> { ... }
    fn<R: T+ToPrimitive> size_hint() -> (uint, Option<uint>) { ... }
    // in this case, size_hint() must either have a default impl, or a second impl using negative trait bounds must be provided
}
