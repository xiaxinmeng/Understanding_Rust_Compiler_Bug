
error[E0623]: lifetime mismatch
 --> src/main.rs:2:5
  |
1 | fn foo<'a, 'b>(a: &'a str, b: &'b str) -> &'b str {
  |                   -------                 -------
  |                   |
  |                   this parameter and the return type are declared with different lifetimes...
2 |     a
  |     ^ ...but data from `a` is returned here
