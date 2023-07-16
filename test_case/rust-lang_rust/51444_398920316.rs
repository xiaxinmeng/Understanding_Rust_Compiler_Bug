
error[E0621]: explicit lifetime required in the type of `x`
 --> file.rs:3:25
  |
2 | fn foo(x: &u32) -> impl Debug {
  |        - consider changing the type of `x` to `&'static u32`
3 |   let _: &'static u32 = x;
  |                         ^ lifetime `'static` required

error[E0621]: explicit lifetime required in the type of `x`
 --> file.rs:7:29
  |
7 | fn bar(x: &u32, y: &u32) -> impl Debug {
  |        -                    ^^^^^^^^^^ lifetime `'static` required
  |        |
  |        consider changing the type of `x` to `&'static u32`
