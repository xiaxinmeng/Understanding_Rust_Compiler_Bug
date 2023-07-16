rust
pub trait ToValues {
    type Val;
    type ValIter<'a>: Iterator<Item = Self::Val> + 'a;

    fn to_values<'a>(
        self,
    ) -> Self::ValIter<'a>;
}
