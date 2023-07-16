diff
  impl<K, V, S> HashMap<K, V, S>
- where
-     K: Eq + Hash,
-     S: BuildHasher,
  {
      fn capacity(&self) -> usize;
      fn keys(&self) -> Keys<K, V>;
      fn values(&self) -> Values<K, V>;
      fn values_mut(&mut self) -> ValuesMut<K, V>;
      fn iter(&self) -> Iter<K, V>;
      fn iter_mut(&mut self) -> IterMut<K, V>;
      fn len(&self) -> usize;
      fn is_empty(&self) -> bool;
      fn drain(&mut self) -> Drain<K, V>;
      fn clear(&mut self);
  }

  impl<'a, K, V, S> IntoIterator for &'a HashMap<K, V, S>
- where
-     K: Eq + Hash,
-     S: BuildHasher;

  impl<'a, K, V, S> IntoIterator for &'a mut HashMap<K, V, S>
- where
-     K: Eq + Hash,
-     S: BuildHasher;

  impl<K, V, S> IntoIterator for HashMap<K, V, S>
- where
-     K: Eq + Hash,
-     S: BuildHasher;

  impl<T, S> HashSet<T, S>
- where
-     T: Eq + Hash,
-     S: BuildHasher,
  {
      fn capacity(&self) -> usize;
      fn iter(&self) -> Iter<T>;
      fn len(&self) -> usize;
      fn is_empty(&self) -> bool;
      fn drain(&mut self) -> Drain<T>;
      fn clear(&mut self);
  }

  impl<'a, T, S> IntoIterator for &'a HashSet<T, S>
- where
-     T: Eq + Hash,
-     S: BuildHasher;

  impl<T, S> IntoIterator for HashSet<T, S>
- where
-     T: Eq + Hash,
-     S: BuildHasher;
