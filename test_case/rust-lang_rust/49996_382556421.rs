
fn bar<'a, I>(self, collection: I) where I: IntoIterator<Item = &'a Self::Baz>
