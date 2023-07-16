
fn remove_range(&mut self, a: uint, b: uint) -> Self {
  // probably some asserts here or whatever
  let tmp1 = self.split_off(a);
  let tmp2 = tmp1.split_off(b - a);
  self.append(tmp2);
  tmp1
}
