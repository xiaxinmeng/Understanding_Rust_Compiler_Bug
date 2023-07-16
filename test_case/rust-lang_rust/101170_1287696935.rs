rust
fn uwu(x: &mut [i32]) {
  let ptr = x.as_ptr();
  x[0] = 0;
  ptr.read(); // UB! The write on the previous line invalidated this pointer.
}