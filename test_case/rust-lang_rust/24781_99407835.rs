
trait IsRange {
    type Idx;
    fn as_range(&self)         -> Range<Option<&Idx>>;
    fn as_mut_range(&mut self) -> Range<Option<&mut Idx>>;
    fn into_range(self)        -> Range<Option<Idx>>;
}
