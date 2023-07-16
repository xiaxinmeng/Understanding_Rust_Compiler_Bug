rust
extern { type T_impl; }

fn foo<T>(a: M<T>) -> R<T> {
  let _2: C<T> = T::CONST;
  let _3: fn(M<T>, C<T>) -> R<T> = T::bar;
  transmute(foo_impl(transmute(a), transmute(_2), transmute(_3)))
}

fn foo_impl(a: M<T_impl>, c: C<T_impl>, _3: fn(M<T_impl>, C<T_impl>) -> R<T_impl>) -> R<T_impl> {
  // stuff with T_impl
  _3(a, c)
}
