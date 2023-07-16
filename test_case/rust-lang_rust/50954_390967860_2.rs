rust
impl<'a, B> Borrow<B> for Cow<'a, B> where B: ToOwned
