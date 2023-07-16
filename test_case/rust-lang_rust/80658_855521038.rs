bash
nell@DESKTOP-VBH9MU6:~/rust/troubleshooting_example$ rustc +rust-stage-1 src/lib.rs --edition 2018
error[E0308]: mismatched types
 --> src/lib.rs:4:5
  |
1 | fn foo() -> u8 {
  |             -- expected `u8` because of return type
2 |     async fn async_fn() -> u8 {  22 }
  |                            -- calling an async function returns a future
3 |
4 |     async_fn()
  |     ^^^^^^^^^^ expected `u8`, found opaque type
  |
  = note: while checking the return type of the `async fn`
  = note:     expected type `u8`
          found opaque type `impl Future`
help: consider `await`ing on the `Future`
  |
4 |     async_fn().await
  |               ^^^^^^

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0308, E0601.
For more information about an error, try `rustc --explain E0308`.
