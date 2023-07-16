 rust
fn main() {
  let mut a = ~1;
  let b = &mut *a;
  a = ~2;
}
// error: cannot assign to `a` because it is borrowed
