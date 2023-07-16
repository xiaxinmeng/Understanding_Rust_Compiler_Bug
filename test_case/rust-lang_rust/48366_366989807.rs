rust
// = () denotes the "I am adding no additional constraints" bound
pub trait Hash<trait Extra = ()> {
    fn hash<H>(&self, state: &mut H) where H: Hasher + Bounds;
}

impl<K: Eq + Hash, V: Hash, S: BuildHasher> Hash<Clone> for HashMap<K, V, S> {
    fn hash<H: Hasher + Clone>(&self, hasher: &mut H) {
        let r = 
            self.iter()
                .map(|kv| { let mut h = hasher.clone(); kv.hash(&mut h); h.finish() })
                .fold(0, u64::wrapping_add);
        hasher.write_u64(r);
    }
}

