rust
pub fn foo(a: &mut i32, b: &mut i32, x: &i32) {
  *a = *x;
  *b = *x;
}
