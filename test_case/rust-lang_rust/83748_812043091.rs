rust
fn dedup(self) -> DedupBy<Self, impl FnMut(&Self::Item, &Self::Item) -> bool, Self::Item>
where Self::Item: PartialEq {
    self.dedup_by(|a, b| a == b)
}
