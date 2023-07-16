rust
fn bar<'a, I>(self, collection: &'a I) where for <'b> &'b I: IntoIterator<Item= &'b &'a Self::Baz> {
    let collection = collection.into_iter().map(|b| &b.delegate).collect::<Vec<&<T as FooMut>::Baz>>();

    self.delegate.bar::<Vec<&<T as FooMut>::Baz>>(&collection)
}
