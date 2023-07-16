rust
fn dedup(self) -> DedupBy<Self, impl FnMut(&T, &T) -> bool, Self::Item>
