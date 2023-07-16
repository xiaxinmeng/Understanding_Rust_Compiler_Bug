 rust
struct FooDecorator<R> {
    inner:R
}
impl<T:Foo,R:ByRef<T>> Foo for FooDecorator<R> {
  fn foo(&self) {
    /* do something... */
    inner.get_ref().foo();
    /* and do something else... */
  }
}
