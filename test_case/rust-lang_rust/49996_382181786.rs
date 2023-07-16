rust
fn bar<'a, I>(self, collection: &'a I) where for <'b> &'b I: IntoIterator<Item= &'b &'a Self::Baz> {
    // same as both examples in above post
    let collection = collection.into_iter().map(|b| &b.delegate).collect::<Vec<&<T as FooMut>::Baz>>();

    // shorter turbofish (infer the element type)
    self.delegate.bar::<Vec<_>>(&collection)
}
