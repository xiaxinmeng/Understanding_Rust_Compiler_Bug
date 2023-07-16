 rust
impl<K:Hash+Eq,V> HashMap<K,V,K,Eq<K>> {
    pub fn into_equiv<ArgKey>(self) -> HashMap<K,V,ArgKey,Equiv<ArgKey>> { ... }
}
