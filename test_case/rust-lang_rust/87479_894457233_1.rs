 rust
trait TableIterator {
  type Iter<'db, K, V>: Iterator<(&'db K, &'db V)>;
  fn iter(&'txn self, db: &'db Db<K, V>) -> Self::Iter<'db, K, V>;
}
