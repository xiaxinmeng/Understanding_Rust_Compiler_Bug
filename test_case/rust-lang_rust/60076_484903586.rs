rust
fn foo(mut x: (T, &[U])) {
  let ptr = &x.1[0] as *const U;
  // do some stuff with x that "semantically" "exhausts" the slice.
  x.1 = &[];
  // then write to where `x.1` used to point to.
  *(ptr as *mut U) = ...;
}
