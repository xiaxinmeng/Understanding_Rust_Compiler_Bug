Rust
#[inline(always)]
fn xor(l: &mut [u8], self_buf: &[u8; 16*8]) {
  for i in 0..16*8 { l[i] ^= self_buf[i]; }
}

pub fn square(mut buf: &mut [u8], self_buf: &[u8; 16*8]) {
  while buf.len() >= self_buf.len() {
      let (l, r) = {buf}.split_at_mut(self_buf.len());
      buf = r;
      xor(l, self_buf);
  }
}
