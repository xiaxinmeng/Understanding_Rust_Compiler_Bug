
error[E0308]: mismatched types
 --> main.rs:2:5
  |
1 |   fn foo() -> i32 {
  |               --- expected `i32` because of return type
2 | /     for i in 0..0 {
3 | |         return i;
4 | |     }
  | |_____^ expected `i32`, found `()`

note: the function expects a value to always be returned, but loops might run zero times
 --> main.rs:2:5
  |
2 |       for i in 0..0 {
  |       ^------------
  |       |
  |  _____this might have zero elements to iterate on
  | |
3 | |         return i;
  | |         -------- if the loop doesn't execute, this value would never get returned
4 | |     }
  | |_____^
  |
  = help: return a value for the case when the loop has zero elements to iterate on, and consider changing the return type to account for that possibility

error: aborting due to previous error

