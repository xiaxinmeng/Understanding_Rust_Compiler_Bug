
impl<T:Eq<T>> Eq<Foo<T>> for Foo<T> {
  fn eq(&self, other: &Foo<T>) { ... <same as now> ... }
}
