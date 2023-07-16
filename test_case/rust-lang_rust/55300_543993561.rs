rust
impl<T> [T] {
    pub fn partition_at_index(&mut self, index: usize) -> (&mut [T], &mut T, &mut [T])
        where T: Ord
    {…}
    pub fn partition_at_index_by<F>(&mut self, index: usize, mut compare: F)
                                    -> (&mut [T], &mut T, &mut [T])
        where F: FnMut(&T, &T) -> Ordering
    {…}
    pub fn partition_at_index_by_key<K, F>(&mut self, index: usize, mut f: F)
                                           -> (&mut [T], &mut T, &mut [T])
        where F: FnMut(&T) -> K, K: Ord
    {…}
}
