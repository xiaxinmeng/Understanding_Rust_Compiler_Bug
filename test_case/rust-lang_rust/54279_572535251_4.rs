rust
fn partition_dedup(&mut self) -> (&mut Self, &mut Self) {
    let p = self.dedup_in_place().len();
    self.split_at_mut(p)
}
