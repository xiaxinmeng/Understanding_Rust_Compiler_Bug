rust
// Could use a better name
pub struct Neighbors<'a, K: 'a, V: 'a> {
    pub lesser: Option<(&K, &V)>,
    pub equal: Option<(&K, &V)>,
    pub greater: Option<(&K, &V)>,
};

pub fn neighbors<Q>(&self, key: &Q) -> Neighbors<K, V>
    where
        K: Borrow<Q> + Ord,
        Q: Ord + ?Sized;
