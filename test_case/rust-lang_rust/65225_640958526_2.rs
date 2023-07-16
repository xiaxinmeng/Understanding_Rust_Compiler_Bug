rust
fn insert(&mut self, msg: Message) -> &Message {
  let msg2 = msg.clone();
  self.map.entry(msg).insert(0); // or just `.or_insert()`
  self.map.get_key_value(&msg2).unwrap().0
