rust
mod m {
  //// Invariant: integer field is always even.
  #[repr(transparent)]
  struct S(i32);
}
