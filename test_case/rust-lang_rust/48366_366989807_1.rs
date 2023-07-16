rust
impl<K, V, S> Hash for HashMap<K, V, S>
    where K: Eq + Hash,
          V: Hash,
          S: BuildHasher
{
    fn hash<H: Hasher + Clone>(&self, hasher: &mut H) {
        // ...
        hasher.write_u64(
            self.iter()
                .map(|kv| {
                    let mut h = self.hash_builder.build_hash(); // <--- look ma!
                    kv.hash(&mut h);
                    h.finish()
                })
                .fold(0, u64::wrapping_add)
        );
    }
}
