rust
// returns Some(value) if KV-pair already exists
pub fn insert_vacant(&mut self, key: K, value: V) -> Option<V>;
