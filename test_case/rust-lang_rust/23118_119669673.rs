 rust
pub trait Container {
    type Key;
    type Value;
}

pub trait MapLookup<Q: ?Sized>: Container {
    fn get(&self, key: &Q) -> Option<&Self::Value>;
}

pub trait Map: MapLookup<<Self as Container>::Key> {
    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value>;
}
