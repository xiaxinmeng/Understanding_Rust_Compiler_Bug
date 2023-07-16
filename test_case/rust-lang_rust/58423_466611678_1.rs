diff
  impl<R> BufReader<R>
- where
-     R: Read,
  {
      pub fn buffer(&self) -> &[u8] { &self.buf[self.pos..self.cap] }
  }
