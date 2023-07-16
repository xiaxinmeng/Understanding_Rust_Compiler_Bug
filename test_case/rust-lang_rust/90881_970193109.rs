plain
[RUSTC-TIMING] build_script_build test:false 0.567
error[E0432]: unresolved import `crate::core_arch::wasm32`
   --> library/core/src/../../stdarch/crates/core_arch/src/mod.rs:162:35
    |
162 |         pub use crate::core_arch::wasm32::*;
    |                                   ^^^^^^ could not find `wasm32` in `core_arch`

error[E0432]: unresolved import `core::arch::wasm32::v128`
 --> library/core/src/../../portable-simd/crates/core_simd/src/vendor/wasm32.rs:2:5
  |
2 | use core::arch::wasm32::v128;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^ no `v128` in `core_arch::arch::wasm32`
For more information about this error, try `rustc --explain E0432`.
[RUSTC-TIMING] core test:false 12.504
error: could not compile `core` due to 2 previous errors
Build completed unsuccessfully in 0:15:40
