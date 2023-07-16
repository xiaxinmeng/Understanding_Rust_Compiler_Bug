
error[E0618]: expected function, found `Result<i32, i32>`
 --> src/main.rs:6:5
  |
1 | fn foo() -> Result<i32, i32> {
  | ---------------------------- `foo` defined here returns `Result<i32, i32>`
...
6 |     foo()();
  |     ^^^^^--
  |     |
  |     call expression requires function
