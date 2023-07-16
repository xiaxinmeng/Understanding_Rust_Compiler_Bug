
trait MapCopy<K:Copy, V> {
    fn get_copy(&'lt self, k: K) -> &'lt V;
    ...
}

impl<K:Copy,V,M:Map<K,V>> MapCopy<K,V> for M {
    fn get_copy(&'lt self, K: K) -> &'lt V {
        self.get(&k)
    }
}
