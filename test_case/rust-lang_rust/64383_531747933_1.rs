rust
fn next(&mut self) -> Option<&'a K> {
  loop {
    let a_next = self.a.next()?;
    let b_next = self.b.skip_while_less_than(a_next)?;
    if a_next == b_next {
      return Some(a_next);
    }
    swap(&mut self.a, &mut self.b); // or swap based on remaining size.
  }
}
