
fn retain_or_drain<F>(&mut self, mut f: F) -> std::vec::Drain<T>
  where
    F: FnMut(&T) -> bool,
{
  // this is exactly the same as the current retain() method...
  let len = self.len();
  let mut del = 0;
  {
    let v = &mut **self;

    for i in 0..len {
      if !f(&v[i]) {
        del += 1;
      } else if del > 0 {
        v.swap(i - del, i);
      }
    }
  }
  //... until here
  // the existing retain() methods truncates
  // this new method would return a drain instead
  self.drain(len - del..)
}
