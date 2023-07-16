rust
fn skip_while_less_than(&mut self, target: &K) -> Option<&K> {
  loop {
    let cur = self.next()?;
    if cur >= target {
      return Some(cur);
    }
  }
}
