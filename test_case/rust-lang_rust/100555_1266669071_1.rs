rust
impl<T, D> Deserialize<Foo<T>, D> for <Foo<T> as Archive>::Archived {
    // ...
}

impl<T> Archive for Foo<T> {
   type Assoc = LocalType;
}
