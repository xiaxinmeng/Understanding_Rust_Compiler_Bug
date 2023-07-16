
pub trait Container {
    // I was always more in favor of 'size' because I only really consider
    // vectors as having a 'length'
    pure fn len(&self) -> uint;
    pure fn is_empty(&self) -> bool;
}

pub trait Mutable: Container {
    fn clear(&mut self);
}

pub trait Map<K, V>: Mutable {
    pure fn contains_key(&self, key: K) -> bool;
    pure fn each(&self, f: fn(K, V) -> bool);
    pure fn each_key(&self, f: fn(K) -> bool);
    pure fn each_value(&self, f: fn(V) -> bool);
    pure fn find(&self, key: K) -> Option<V>;
    fn remove(&mut self, key: K) -> bool;
}

pub trait GrowableMap<K, V> {
    fn insert(&mut self, key: K, value: V) -> bool;
}

impl<K, V> Map<&K, &V> for LinearMap<K, V> { ... }
impl<K, V> GrowableMap<K, V> for LinearMap<K, V> { ... }

impl<V> Map<int, &V> for SmallIntMap<V> { ... }
impl<V> GrowableMap<int, V> for SmallIntMap<V> { ... }
