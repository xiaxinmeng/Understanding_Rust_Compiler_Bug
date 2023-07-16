 rust
impl Iterator for FixedRangeInclusive {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let new = self.start.wrapping_add(1);
            Some(std::mem::replace(&mut self.start, new))
        } else if !self.done {
            self.done = true;
            Some(self.start)
        } else {
            None
        }
    }
}
