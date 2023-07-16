
error[E0621]: explicit lifetime required in the type of `x`
 --> src/lib.rs:4:13
  |
2 |     fn foo<'a>(x: &i32, y: &'a i32) -> &'a i32 {
  |                   ---- help: add explicit lifetime `'a` to the type of `x`: `&'a i32`
3 |         if x > y {
4 |             x
  |             ^ lifetime `'a` required

error: aborting due to previous error
