
error[E0618]: expected function, found `Result<fn() {empty}, ()>`
 --> src/main.rs:5:5
  |
4 |     let f: Result<_, ()> = Ok(empty);
  |         - `f` has type `Result<fn() {empty}, ()>`
5 |     f();
  |     ^--
  |     |
  |     call expression requires function
