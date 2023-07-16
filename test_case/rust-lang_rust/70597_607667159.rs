
error: struct is never constructed: `Handler`
 --> src/libstd/sys/wasm/stack_overflow.rs:1:12
  |
1 | pub struct Handler;
  |            ^^^^^^^
  |
  = note: `-D dead-code` implied by `-D warnings`

error: method is never used: `new`
 --> src/libstd/sys/wasm/stack_overflow.rs:4:5
  |
4 |     pub unsafe fn new() -> Handler {
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to 2 previous errors
