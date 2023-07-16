rust
impl<Idx : Copy> From<Range<Idx>> for CopyRange<Idx> { /* ... */ }
impl<Idx : Copy + Step> IntoIterator for CopyRange<Idx> {
    type IntoIter = Range<Idx>;
    // ...
}
