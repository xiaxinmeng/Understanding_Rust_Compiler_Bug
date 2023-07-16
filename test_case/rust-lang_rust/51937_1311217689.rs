
error[E0308]: mismatched types
 --> src/lib.rs:3:5
  |
2 | /   if helper_fn() == 0 {
3 | |     10
  | |     ^^ expected `()`, found integer
4 | |   } else {
5 | |     20
6 | |   }
  | |___- expected this to be `()`
  |
help: you might have meant to return this value
  |
3 |     return 10;
  |     ++++++   +

error[E0308]: mismatched types
 --> src/lib.rs:5:5
  |
2 | /   if helper_fn() == 0 {
3 | |     10
4 | |   } else {
5 | |     20
  | |     ^^ expected `()`, found integer
6 | |   }
  | |___- expected this to be `()`
  |
help: you might have meant to return this value
  |
5 |     return 20;
  |     ++++++   +

error[E0308]: mismatched types
 --> src/lib.rs:1:20
  |
1 | fn example_fn() -> i32 {
  |    ----------      ^^^ expected `i32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
