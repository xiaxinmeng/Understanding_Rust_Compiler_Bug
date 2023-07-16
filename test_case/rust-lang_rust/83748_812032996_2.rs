rust
fn dedup(self) -> DedupBy<Self, impl for<'a> FnMut(&'a T, &'a T) -> bool, Self::Item>
