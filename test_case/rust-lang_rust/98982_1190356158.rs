
error[E0308]: mismatched types
 --> src/lib.rs:2:5
  |
1 |   fn foo() -> i32 {
  |               --- expected `i32` because of return type
2 | /     for i in 0..0 {
   |           ^^^^ this might have zero elements to iterate on
3 | |         return i;
   |                - if the loop doesn't execute, this value would never get returned
help: return a value for the case when the loop has zero elements to iterate on, and consider changing the return type to account for that possibility
4 | |     }
  | |_____^ expected `i32`, found `()`
note: the function expects a value to always be returned, but loops might run zero times
