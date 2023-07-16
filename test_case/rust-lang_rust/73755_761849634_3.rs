
rustc -v --crate-type=cdylib -C opt-level=1 --target wasm32-unknown-unknown -C target-feature=+multivalue --emit obj,llvm-ir test.rs
warning: `extern` fn uses type `(i32, i32)`, which is not FFI-safe
 --> test.rs:2:40
  |
2 | pub extern fn greet(x: i32, y: i32) -> (i32, i32) {
  |                                        ^^^^^^^^^^ not FFI-safe
  |
  = note: `#[warn(improper_ctypes_definitions)]` on by default
  = help: consider using a struct instead
  = note: tuples have unspecified layout

warning: 1 warning emitted
