
Compiling playground v0.0.1 (/playground)
error[[E0308]](https://doc.rust-lang.org/stable/error-index.html#E0308): mismatched types
 --> src/lib.rs:4:5
  |
1 | fn foo() -> u8 {
  |             -- expected `u8` because of return type
...
4 |     async_fn()
  |     ^^^^^^^^^^ expected `u8`, found opaque type
  |
note: while checking the return type of the `async fn`
 --> src/lib.rs:2:28
  |
2 |     async fn async_fn() -> u8 {  22 }
  |                            ^^ checked the `Output` of this `async fn`, found opaque type
  = note:     expected type `u8`
          found opaque type `impl Future<Output = u8>`
help: consider `await`ing on the `Future`
  |
4 |     async_fn().await
  |               ++++++

For more information about this error, try `rustc --explain E0308`.
error: could not compile `playground` due to previous error
