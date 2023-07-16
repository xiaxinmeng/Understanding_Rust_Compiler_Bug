
#[inline(always)]
pub fn sort_unstable_by_cached_key<K, F>(&mut self, f: F)
    where F: FnMut(&T) -> K, K: Ord
{
    self.sort_by_cached_key(f)
}
