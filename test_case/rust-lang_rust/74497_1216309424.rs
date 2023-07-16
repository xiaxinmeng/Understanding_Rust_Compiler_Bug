
error[E0308]: mismatched types
  --> src/lib.rs:4:5
   |
4  |     foo(baz).await;
   |     ^^^^^^^^ one type is more general than the other
   |
   = note: expected trait `for<'r> <for<'r> fn(&'r u8) -> impl for<'r> Future<Output = bool> {baz} as FnOnce<(&'r u8,)>>`
              found trait `for<'r> <for<'r> fn(&'r u8) -> impl for<'r> Future<Output = bool> {baz} as FnOnce<(&'r u8,)>>`
note: the lifetime requirement is introduced here
  --> src/lib.rs:17:19
   |
17 |     F: Fn(&u8) -> T,
   |                   ^
