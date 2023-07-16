rust
fn insert(&mut self, msg: Message) -> &Message {
  self.map.entry(msg).insert(0).key()
