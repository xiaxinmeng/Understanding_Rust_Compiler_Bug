 rust

pub struct HashWithKey<K, 'a> {
  priv hash: SafeHash;
  priv key: &'a K;
}

impl<K, 'a> HashWithKey<K, 'a> {
  pub fn key(&self) -> &'a K;
  fn hash(&self) -> &'a SafeHash;
}

pub fn hash<'a>(&self, k: &'a K) -> HashWithKey<K, 'a>
