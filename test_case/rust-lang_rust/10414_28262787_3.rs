
impl<T> Iterator<T> for Range<T> {
    fn next() -> Option<T> { ... }
    if!<T: ToPrimitive> {
        fn size_hint() -> (uint, Option<uint>) { ... }
    }
}
