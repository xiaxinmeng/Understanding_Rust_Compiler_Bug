rust
fn test(v: &mut *mut u32, x: mut u32) {
  unsafe { *v.add(1) = 1; }
  *x = 2;
}
