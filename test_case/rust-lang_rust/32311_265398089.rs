rust
impl<Idx, T> Range<Idx> where T:PartialOrd<Idx>, Idx:PartialOrd<T> {
    pub fn contains(&self, item: T) -> bool {
        (self.start <= item) && (item < self.end)
    }
}
