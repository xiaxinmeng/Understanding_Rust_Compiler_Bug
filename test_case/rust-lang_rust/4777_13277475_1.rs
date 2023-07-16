
pub trait PoppableMap<K, V> {
    fn pop(&mut self, key: K) -> Option<V>;
}

impl<K, V> PoppableMap<&K, V> for LinearMap<K, V> { ... }
impl<V> PoppableMap<uint, V> for SmallIntMap<V> { ... }
