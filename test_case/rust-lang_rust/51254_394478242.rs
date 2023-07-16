rust
fn nth(&mut self, n: usize) -> Option<Self::Item> {
    let (start, overflow) = n.overflowing_mul(self.chunk_size);
    if start >= self.v.len() || overflow {
// ...
