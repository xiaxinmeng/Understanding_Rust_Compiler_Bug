rust
  EndLifetime('a);
  T<'static>::drop(&mut x);
  mem::forget(x);
