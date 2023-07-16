rust
impl<Idx: PartialOrd<Idx>> Range<Idx> {
    pub fn is_empty(&self) -> bool;
}

impl<Idx: PartialOrd<Idx>> RangeInclusive<Idx> {
    pub fn is_empty(&self) -> bool;
}
