
error[E0621]: explicit lifetime required in the type of `x`
 --> src/main.rs:2:5
  |
1 | fn foo<'a>(x: &i32) -> &'a i32 {
  |            - consider changing the type of `x` to `&'a i32`
2 |     x
  |     ^ lifetime `'a` required
