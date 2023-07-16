
error[E0623]: lifetime mismatch
 --> src/main.rs:2:10
  |
1 | async fn async_fn(x: &i32, y: &mut &i32) {
  |                                    ----  -
  |                                    |
  |                                    this parameter and the return type are declared with different lifetimes...
2 |     *y = x;
  |          ^ ...but data from `x` is returned here

error[E0623]: lifetime mismatch
 --> src/main.rs:6:10
  |
5 | fn sync_fn(x: &i32, y: &mut &i32) {
  |               ----          ----
  |               |
  |               these two types are declared with different lifetimes...
6 |     *y = x;
  |          ^ ...but data from `x` flows into `y` here
