 rust
pub trait Lookup<Q: ?Sized> {
    type Value;
    fn get(&self, key: &Q) -> Option<&Self::Value>;
}
