bash
$ cargo build

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
          found opaque type `impl std::future::Future`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: could not compile `troubleshooting_example`.

To learn more, run the command again with --verbose.
