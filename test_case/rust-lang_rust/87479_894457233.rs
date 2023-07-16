 rust
trait TableIterator {
  type Iter<'txn, K, V>: Iterator<(&'txn K, &'txn V)>;
  fn iter(&'txn self, db: &'txn Db<K, V>) -> Self::Iter<'txn, K, V>;
}
