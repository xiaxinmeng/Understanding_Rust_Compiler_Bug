rust
let mut gen = || {
  let mut x: (i32, i32);
  x.0 = 11;
  yield;
  x.1 = 22;
}
