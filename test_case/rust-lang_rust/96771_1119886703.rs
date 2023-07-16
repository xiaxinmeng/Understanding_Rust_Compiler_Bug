rust
async fn foo<'a>(
  a: &'static i32,
  b: &'a i32,
  c: &'a i32,
) -> Option<&'static i32> {
  None
}
