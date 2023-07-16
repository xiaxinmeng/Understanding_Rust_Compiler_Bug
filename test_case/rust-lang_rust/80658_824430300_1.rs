
error[E0308]: mismatched types
 --> src/lib.rs:4:5
  |
1 | fn foo() -> u8 {
  |             -- expected `u8` because of return type
2 |     async fn async_fn() -> u8 {  22 }
  |                            -- the `Output` of this `async fn`'s found opaque type
3 |     
4 |     async_fn()
  |     ^^^^^^^^^^ expected `u8`, found opaque type
  |
  = note:     expected type `u8`
          found opaque type `impl Future`
help: consider `await`ing on the `Future`
  |
4 |     async_fn().await
  |               ^^^^^^
