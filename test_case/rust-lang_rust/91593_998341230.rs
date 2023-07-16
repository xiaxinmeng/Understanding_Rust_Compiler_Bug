diff
  impl<K, V, S> HashMap<K, V, S> {
      pub fn into_keys(self) -> IntoKeys<K, V>
      where
-         K: Eq + Hash,
-         S: BuildHasher;

      pub fn into_values(self) -> IntoValues<K, V>
      where
-         K: Eq + Hash,
-         S: BuildHasher;

      pub fn retain<F>(&mut self, f: F)
      where
-         K: Eq + Hash,
-         S: BuildHasher,
          F: FnMut(&K, &mut V) -> bool;
  }

  impl<T, S> HashSet<T, S> {
      pub fn retain<F>(&mut self, f: F)
      where
-         T: Eq + Hash,
-         S: BuildHasher,
          F: FnMut(&T) -> bool;
  }
