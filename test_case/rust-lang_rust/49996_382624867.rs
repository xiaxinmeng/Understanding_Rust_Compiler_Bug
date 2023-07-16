rust
fn bar<'a, I, B>(self, collection: I) where I: IntoIterator<Item=B>, B: Borrow<&'a Self::Baz>
