rust
pub fn try_insert(&mut self, key: K, value: V) -> Result<Option<V>, TryReserveError>;
