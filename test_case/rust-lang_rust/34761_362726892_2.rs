rust
  T<'a>::drop(&mut x);
  EndLifetime('a);
  mem::forget(x);
