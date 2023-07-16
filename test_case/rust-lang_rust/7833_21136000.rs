 rust
fn consume(&mut self) -> SomeIterator<(K, V)> {
  let arr = util::replace(&mut self.slots, ~[]);
  arr.consume_iter().filter_map(|x| match x { Some(slot) => Some((slot.key, slot.value)), None => None })
}
