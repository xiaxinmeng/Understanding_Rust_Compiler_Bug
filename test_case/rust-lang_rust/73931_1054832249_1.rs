
error[E0623]: lifetime mismatch
 --> src/main.rs:7:9
  |
6 |     fn foo(&self, x: &i32, y: &i32) -> Option<&i32> {
  |                               ----     ------------
  |                               |
  |                               this parameter and the return type are declared with different lifetimes...
7 |         Some(y)
  |         ^^^^^^^ ...but data from `y` is returned here
  |
  = note: each elided lifetime in input position becomes a distinct lifetime
help: consider introducing a named lifetime parameter
  |
6 |     fn foo<'a>(&'a self, x: &i32, y: &'a i32) -> Option<&i32> {
  |           ++++  ++                    ++

error: aborting due to previous error

For more information about this error, try `rustc --explain E0623`.

