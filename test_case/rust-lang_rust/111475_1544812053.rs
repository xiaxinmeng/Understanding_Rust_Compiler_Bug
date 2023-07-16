plain
[RUSTC-TIMING] cc test:false 0.603
[RUSTC-TIMING] build_script_build test:false 0.468
   Compiling core v0.0.0 (/checkout/library/core)
   Compiling compiler_builtins v0.1.91
error[E0432]: unresolved import `core::arch::arm::vtbl1_u8`
  --> library/core/src/../../portable-simd/crates/core_simd/src/swizzle_dyn.rs:22:42
   |
22 |         use core::arch::arm::{uint8x8_t, vtbl1_u8};
   |                                          ^^^^^^^^ no `vtbl1_u8` in `core_arch::arch::arm`
For more information about this error, try `rustc --explain E0432`.
[RUSTC-TIMING] core test:false 6.750
error: could not compile `core` (lib) due to previous error
Build completed unsuccessfully in 0:19:30
