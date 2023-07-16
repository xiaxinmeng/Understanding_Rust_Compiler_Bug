rust
// impl DoubleEndedIteator ...
pub fn nth_back(&mut self, n: usize) -> Option<&'a [T]> {
    let (end, overflow) = self.v.len().overflowing_sub(n);
    if end < self.size.get() || overflow {
        self.v = &[];
        None
    } else {
        let ret = &self.v[end - self.size.get()..end];
        self.v = &self.v[..end - 1];
        Some(ret)
    }
}
