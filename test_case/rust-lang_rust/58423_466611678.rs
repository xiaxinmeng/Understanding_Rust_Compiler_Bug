diff
  impl<R> BufReader<R>
- where
-     R: Read,
  {
      pub fn get_ref(&self) -> &R { &self.inner }
      pub fn get_mut(&mut self) -> &mut R { &mut self.inner }
      pub fn into_inner(self) -> R { self.inner }
  }
