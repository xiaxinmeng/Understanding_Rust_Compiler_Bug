rust
enum Foo {
  A(i32)
  B(i32, #[doc(hidden)] i32)
  C(#[doc(hidden)] i32, i32)
}
