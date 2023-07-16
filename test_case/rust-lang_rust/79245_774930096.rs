diff
  impl<K, V> BTreeMap<K, V> {
      pub fn clear(&mut self)
-     where
-         K: Ord;
  }

  impl<T> BTreeSet<T> {
      pub fn clear(&mut self)
-     where
-         T: Ord;
  }
