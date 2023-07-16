
test_wasm on  master [?] is 📦 v0.1.0 via 🦀 v1.46.0
❯ rustc --crate-type cdylib --target wasm32-unknown-unknown src/lib.rs -C opt-level=3 --emit obj -C target-feature=+multivalue
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


test_wasm on  master [?] is 📦 v0.1.0 via 🦀 v1.46.0
❯ wasm-objdump -d lib.o

lib.o:  file format wasm 0x1

Code Disassembly:

000066 func[0] <magic>:
 000067: 20 01                      | local.get 1
 000069: 20 00                      | local.get 0
 00006b: 6a                         | i32.add
 00006c: 20 00                      | local.get 0
 00006e: 20 01                      | local.get 1
 000070: 6b                         | i32.sub
 000071: 0b                         | end
