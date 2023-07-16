rust
pub trait Iterator {
    fn dedup(self) -> Dedup<Self, ByPartialEq, Self::Item>
    where
        Self: Sized,
        Self::Item: PartialEq,
    { ... }
    
    fn dedup_by<F>(self, same_bucket: F) -> Dedup<Self, F, Self::Item>
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> bool,
    { ... }
    
    fn dedup_by_key<F, K>(self, key: F) -> Dedup<Self, ByKey<F>, Self::Item>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> K,
        K: PartialEq,
    { ... }
}
