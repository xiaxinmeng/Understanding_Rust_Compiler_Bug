diff
  impl<K, V> BTreeMap<K, V> {
      pub fn into_keys(self) -> IntoKeys<K, V>
-     where
-         K: Ord;
- 
      pub fn into_values(self) -> IntoValues<K, V>
-     where
-         K: Ord;
  }
