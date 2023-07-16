plain
[RUSTC-TIMING] gimli test:false 4.276
[RUSTC-TIMING] object test:false 4.538
warning: dropping unsupported crate type `dylib` for target `wasm32-unknown-unknown`

error: unused variable: `x`
  --> library/std/src/sys/wasm/../unsupported/io.rs:49:23
   |
49 | pub fn is_terminal<T>(x: &T) -> bool {
   |                       ^ help: if this is intentional, prefix it with an underscore: `_x`
   |
   = note: `-D unused-variables` implied by `-D warnings`
[RUSTC-TIMING] std test:false 2.778
warning: `std` (lib) generated 1 warning
error: could not compile `std` due to previous error; 1 warning emitted
Build completed unsuccessfully in 0:11:53
