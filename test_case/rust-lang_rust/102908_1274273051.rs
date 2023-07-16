plain
[RUSTC-TIMING] gimli test:false 4.486
[RUSTC-TIMING] object test:false 5.230
warning: dropping unsupported crate type `dylib` for target `wasm32-wasi`

error[E0277]: the trait bound `(): Drop` is not satisfied
  --> library/std/src/sys/wasi/os.rs:40:35
   |
40 |         pub fn env_read_lock() -> impl Drop {
   |                                   ^^^^^^^^^ the trait `Drop` is not implemented for `()`
41 |             ()
   |             -- return type was inferred to be `()` here

error[E0277]: the trait bound `(): Drop` is not satisfied
  --> library/std/src/sys/wasi/os.rs:43:36
   |
43 |         pub fn env_write_lock() -> impl Drop {
   |                                    ^^^^^^^^^ the trait `Drop` is not implemented for `()`
44 |             ()
   |             -- return type was inferred to be `()` here
For more information about this error, try `rustc --explain E0277`.
[RUSTC-TIMING] std test:false 2.310
warning: `std` (lib) generated 1 warning
error: could not compile `std` due to 2 previous errors; 1 warning emitted
