diff
  impl<K, V> BTreeMap<K, V> {
      pub const fn new() -> BTreeMap<K, V>
-     where
-         K: Ord;

      pub fn clear(&mut self)
-     where
-         K: Ord;
  }

  impl<K, V> Default for BTreeMap<K, V>
- where
-     K: Ord;
  }

  impl<K, V> Debug for Entry<'_, K, V>
  where
-     K: Ord + Debug,
+     K: Debug,
      V: Debug;
  }

  impl<K, V> Debug for VacantEntry<'_, K, V>
  where
-     K: Ord + Debug,
+     K: Debug;
  }

  impl<K, V> Debug for OccupiedEntry<'_, K, V>
  where
-     K: Ord + Debug,
+     K: Debug,
      V: Debug;
  }

  impl<'a, K, V> Entry<'a, K, V> {
      pub fn key(&self) -> &K
-     where
-         K: Ord;
  }

  impl<'a, K, V> VacantEntry<'a, K, V> {
      pub fn key(&self) -> &K
-     where
-         K: Ord;

      pub fn into_key(self) -> K
-     where
-         K: Ord;
  }

  impl<'a, K, V> OccupiedEntry<'a, K, V> {
      pub fn key(&self) -> &K
-     where
-         K: Ord;

      pub fn get(&self) -> &V
-     where
-         K: Ord;

      pub fn get_mut(&mut self) -> &mut V
-     where
-         K: Ord;

      pub fn into_mut(self) -> &'a mut V
-     where
-         K: Ord;
  }

  impl<T> BTreeSet<T> {
      pub const fn new() -> BTreeSet<T>
-     where
-         T: Ord;

      pub fn clear(&mut self)
-     where
-         T: Ord;
  }

  impl<T> Default for BTreeSet<T>
- where
-     T: Ord;
