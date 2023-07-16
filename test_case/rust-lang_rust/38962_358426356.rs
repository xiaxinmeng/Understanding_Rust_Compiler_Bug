rust
pub enum Cow<'a, B, O = <B as ToOwned>::Owned> where
    B: 'a + ?Sized,
    O: Borrow<B>
{
    Borrowed(&'a B),
    Owned(O),
}
