
warning: dropping unsupported crate type `dylib` for target `wasm32-unknown-unknown`

error: unnecessary `unsafe` block
  --> library/std/src/sys/wasm/../unsupported/common.rs:43:5
   |
43 |     unsafe {
   |     ^^^^^^ unnecessary `unsafe` block
   |
   = note: `-D unused-unsafe` implied by `-D warnings`

error: aborting due to previous error; 1 warning emitted

[RUSTC-TIMING] std test:false 2.295
error: could not compile `std`
