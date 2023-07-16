
error[E0369]: binary operation `>` cannot be applied to type `fn() -> i32 {foo}`
 --> src/main.rs:6:9
  |
6 |     foo > 12;
  |     --- ^ -- {integer}
  |     |
  |     fn() -> i32 {foo}
  |     help: you might have forgotten to call this function: `foo()`
