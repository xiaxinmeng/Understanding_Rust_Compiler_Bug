rust
fn foo<T>(a: M<T>) -> R<T> {
  let _2: C<T> = T::CONST;
  let _3: fn(M<T>, C<T>) -> R<T> = T::bar;
  foo_impl<T>(a, _2, _3)
}

fn foo_impl<T>(a: M<T>, c: C<T>, _3: fn(M<T>, C<T>) -> R<T>) -> R<T> {
  // stuff with T
  _3(a, c)
}
