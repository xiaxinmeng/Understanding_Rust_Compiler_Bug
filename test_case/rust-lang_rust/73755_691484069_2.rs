
❯ cargo +nightly rustc --release --target wasm32-unknown-unknown -- -C target-feature=+multivalue
   Compiling test_wasm v0.1.0 (/tmp/test_wasm)
warning: `extern` fn uses type `(i32, i32)`, which is not FFI-safe
 --> src/lib.rs:2:44
  |
2 | pub extern "C" fn magic(a: i32, b: i32) -> (i32, i32) {
  |                                            ^^^^^^^^^^ not FFI-safe
  |
  = note: `#[warn(improper_ctypes_definitions)]` on by default
  = help: consider using a struct instead
  = note: tuples have unspecified layout

warning: 1 warning emitted

    Finished release [optimized] target(s) in 0.09s

test_wasm on  master [?] is 📦 v0.1.0 via 🦀 v1.46.0
❯ wasm2wat target/wasm32-unknown-unknown/release/test_wasm.wasm
0000010: error: invalid function result count 2, only 1 bytes left in section
