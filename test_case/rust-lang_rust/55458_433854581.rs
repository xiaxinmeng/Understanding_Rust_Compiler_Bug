rust
pub fn square(mut buf: &mut [u8], self_buf: &[u8; 128]) {
  while buf.len() >= self_buf.len() {
      let (l, r) = {buf}.split_at_mut(self_buf.len());
      buf = r;
      for (a, b) in l.iter_mut().zip(self_buf.iter()) {
        *a ^= *b;
      }
  }
}
