rust
fn foo<T>(a: M<T>) -> R<T> {
  let c: C<T> = T::CONST;
  // stuff with T
  T::bar(a, c)
}
