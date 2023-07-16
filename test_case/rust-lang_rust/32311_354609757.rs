rust
impl<Idx> Range<Idx> {
    pub fn contains<T>(&self, item: &T) -> bool
            where Idx: PartialOrd, Idx: PartialOrd<T> {
        if self.start <= self.end {
            self.start <= *item && self.end > *item
        } else {
            self.start <= *item || self.end > *item
        }
    }
}
