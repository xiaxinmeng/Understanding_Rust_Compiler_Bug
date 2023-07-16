
error[E0308]: missing await on future
 --> src/main.rs:8:12
  |
3 | async fn return_number() -> usize {
  | ----- calling an async function returns a future
...
8 |     return return_number();
  |            ^^^^^^^^^^^^^^^ expected `usize`, found future
  |
help: consider `await`ing on the `Future`
  |
8 |     return return_number().await;
  |                           ^^^^^^

error: aborting due to previous error
