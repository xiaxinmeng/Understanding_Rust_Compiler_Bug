
error[E0308]: missing await on future
 --> src/main.rs:8:12
  |
1 | fn foo() -> u8 {
  |             -- expected `u8` because of return type
2 |     async fn async_fn() -> u8 {  22 }
  |     ----- calling an async function returns a future
3 |     
4 |     async_fn()
  |     ^^^^^^^^^^ expected `u8`, found future
  |
help: consider `await`ing on the `Future`
  |
4 |     async_fn().await
  |               ^^^^^^

error: aborting due to previous error
