 rust
fn zero2ten() -> Zero2TenIter { Zero2TenIter { start: 0, end: 10 } }
struct Zero2TenIter { start: uint, end: uint }
impl Iterator<uint> for Zero2TenIter {
    fn next(&mut self) -> Option<uint> {
        if self.start <= self.end {
            let v = self.start;
            self.start += 1;
            Some(v)
        } else {
            None
        }
    }
}
impl DoubleEndedIterator<uint> for Zero2TenIter {
    fn next_back(&mut self) -> Option<uint> {
        if self.start <= self.end {
            let v = self.end;
            self.end -= 1;
            Some(v)
        } else {
            None
        }
    }
}
