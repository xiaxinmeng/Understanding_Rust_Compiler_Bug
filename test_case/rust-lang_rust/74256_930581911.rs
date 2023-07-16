
   Compiling playground v0.0.1 (/playground)
error[E0623]: lifetime mismatch
 --> src/main.rs:5:9
  |
4 |     async fn async_fn(self: &Struct, f: &u32) -> &u32 {
  |                             -------              ----
  |                             |                    |
  |                             |                    this `async fn` implicitly returns an `impl Future<Output = &u32>`
  |                             this parameter and the returned future are declared with different lifetimes...
5 |         f
  |         ^ ...but data from `f` is held across an await point here

error[E0623]: lifetime mismatch
 --> src/main.rs:9:9
  |
8 |     fn sync_fn(self: &Struct, f: &u32) -> &u32 {
  |                                  ----     ----
  |                                  |
  |                                  this parameter and the return type are declared with different lifetimes...
9 |         f
  |         ^ ...but data from `f` is returned here

For more information about this error, try `rustc --explain E0623`.
error: could not compile `playground` due to 2 previous errors
