
error[E0621]: explicit lifetime required in the type of `x`
 --> src/main.rs:4:5
  |
3 | fn foo<'a>(x: &u32) -> &'a u32 {
  |            - consider changing the type of `x` to `&'a u32`
4 |     &*x
  |     ^^^ lifetime `'a` required
