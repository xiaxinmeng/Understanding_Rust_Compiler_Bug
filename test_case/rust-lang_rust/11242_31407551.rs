 rust
type HashMapKeyIterator<'a, K, V> = Map<'static, (&'a K, &'a V), &'a V, HashMapIterator<'a, K, V>>;

pub fn keys<'a>(&'a self) -> HashMapKeyIterator<'a, K, V> {           
    self.iter().map(|(_, v)| v)                                         
}                                                                       
