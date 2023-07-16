diff
  impl<K, V> BTreeMap<K, V> {
      pub const fn new() -> BTreeMap<K, V>
-     where
-         K: Ord;

      pub fn clear(&mut self)
-     where
-         K: Ord;

      pub fn get<Q>(&self, key: &Q) -> Option<&V>
      where
-         K: Ord + Borrow<Q>,
+         K: Borrow<Q>,
          Q: ?Sized + Ord;

      pub fn get_key_value<Q>(&self, k: &Q) -> Option<(&K, &V)>
      where
-         K: Ord + Borrow<Q>,
+         K: Borrow<Q>,
          Q: ?Sized + Ord;

      pub fn contains_key<Q>(&self, key: &Q) -> bool
      where
-         K: Ord + Borrow<Q>,
+         K: Borrow<Q>,
          Q: ?Sized + Ord;

      pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
      where
-         K: Ord + Borrow<Q>,
+         K: Borrow<Q>,
          Q: ?Sized + Ord;

      pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
      where
-         K: Ord + Borrow<Q>,
+         K: Borrow<Q>,
          Q: ?Sized + Ord;

      pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
      where
-         K: Ord + Borrow<Q>,
+         K: Borrow<Q>,
          Q: ?Sized + Ord;

      pub fn range<T, R>(&self, range: R) -> Range<'_, K, V>
      where
-         K: Ord + Borrow<T>,
+         K: Borrow<T>,
          T: ?Sized + Ord,
          R: RangeBounds<T>;

      pub fn range_mut<T, R>(&mut self, range: R) -> RangeMut<'_, K, V>
      where
-         K: Ord + Borrow<T>,
+         K: Borrow<T>,
          T: ?Sized + Ord,
          R: RangeBounds<T>;

      pub fn split_off<Q>(&mut self, key: &Q) -> Self
      where
-         K: Ord + Borrow<Q>,
+         K: Borrow<Q>,
          Q: ?Sized + Ord;
}

  impl<K, V> Default for BTreeMap<K, V>
- where
-     K: Ord;

  impl<K, V> Debug for Entry<'_, K, V>
  where
-     K: Ord + Debug,
+     K: Debug,
      V: Debug;

  impl<K, V> Debug for VacantEntry<'_, K, V>
  where
-     K: Ord + Debug,
+     K: Debug;

  impl<K, V> Debug for OccupiedEntry<'_, K, V>
  where
-     K: Ord + Debug,
+     K: Debug,
      V: Debug;

  impl<'a, K, V> Entry<'a, K, V> {
      pub fn or_insert(self, default: V) -> &'a mut V
-     where
-         K: Ord;

      pub fn or_insert_with<F>(self, default: F) -> &'a mut V
      where
-         K: Ord,
          F: FnOnce() -> V;

      pub fn key(&self) -> &K
-     where
-         K: Ord;

      pub fn and_modify<F>(self, f: F) -> Self
      where
-         K: Ord,
          F: FnOnce(&mut V);

      pub fn or_default(self) -> &'a mut V
      where
-         K: Ord,
          V: Default;
  }

  impl<'a, K, V> VacantEntry<'a, K, V> {
      pub fn key(&self) -> &K
-     where
-         K: Ord;

      pub fn into_key(self) -> K
-     where
-         K: Ord;

      pub fn insert(self, value: V) -> &'a mut V
-     where
-         K: Ord;
  }

  impl<'a, K, V> OccupiedEntry<'a, K, V> {
      pub fn key(&self) -> &K
-     where
-         K: Ord;

      pub fn remove_entry(self) -> (K, V)
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

      pub fn insert(&mut self, value: V) -> V
-     where
-         K: Ord;

      pub fn remove(self) -> V
-     where
-         K: Ord;
  }

  impl<T> BTreeSet<T> {
      pub const fn new() -> BTreeSet<T>
-     where
-         T: Ord;

      pub fn range<K, R>(&self, range: R) -> Range<'_, T>
      where
-         T: Ord + Borrow<K>,
+         T: Borrow<K>,
          K: ?Sized + Ord,
          R: RangeBounds<K>;

      pub fn symmetric_difference<'a>(&'a self, other: &'a BTreeSet<T>) -> SymmetricDifference<'a, T>
-     where
-         T: Ord;

      pub fn union<'a>(&'a self, other: &'a BTreeSet<T>) -> Union<'a, T>
-     where
-         T: Ord;

      pub fn clear(&mut self)
-     where
-         T: Ord;

      pub fn contains<Q>(&self, value: &Q) -> bool
      where
+         T: Ord + Borrow<Q>,
-         T: Borrow<Q>,
          Q: ?Sized + Ord;

      pub fn remove<Q>(&mut self, value: &Q) -> bool
      where
-         T: Ord + Borrow<Q>,
+         T: Borrow<Q>,
          Q: ?Sized + Ord;

      pub fn split_off<Q>(&mut self, key: &Q) -> Self
      where
-         T: Ord + Borrow<Q>,
+         T: Borrow<Q>,
          Q: ?Sized + Ord;
  }

  impl<T> Default for BTreeSet<T>
- where
-     T: Ord;
