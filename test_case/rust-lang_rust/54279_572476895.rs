rust
impl<T> [T] {
    pub fn partition_dedup(&mut self) -> (&mut [T], &mut [T])
    where
        T: PartialEq,
    {
        self.partition_dedup_by(|a, b| a == b)
    }

    pub fn partition_dedup_by<F>(&mut self, mut same_bucket: F) -> (&mut [T], &mut [T])
    where
        F: FnMut(&mut T, &mut T) -> bool,
    {…}

    pub fn partition_dedup_by_key<K, F>(&mut self, mut key: F) -> (&mut [T], &mut [T])
    where
        F: FnMut(&mut T) -> K,
        K: PartialEq,
    {
        self.partition_dedup_by(|a, b| key(a) == key(b))
    }
}
