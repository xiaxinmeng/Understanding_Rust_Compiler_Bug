
error[E0308]: mismatched types
 --> src/lib.rs:4:5
  |
1 | fn foo() -> u8 {
  |             -- expected `u8` because of return type
2 |     async fn async_fn() -> () {}
  |                            -- checked the `Output` of this `async fn`, found opaque type
3 |     
4 |     async_fn()
  |     ^^^^^^^^^^ expected `u8`, found opaque type
  |
  = note: while checking the return type of the `async fn`
  = note:     expected type `u8`
          found opaque type `impl Future`
