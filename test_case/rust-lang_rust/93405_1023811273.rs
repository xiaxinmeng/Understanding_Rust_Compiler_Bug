plain
    Checking rustc_codegen_gcc v0.1.0 (/checkout/compiler/rustc_codegen_gcc)
error: unused import: `IntPredicate`
 --> src/intrinsic/mod.rs:7:33
  |
7 | use rustc_codegen_ssa::common::{IntPredicate, span_invalid_monomorphization_error};
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`

error: unused import: `rustc_target::abi::HasDataLayout`
   |
   |
15 | use rustc_target::abi::HasDataLayout;

error: could not compile `rustc_codegen_gcc` due to 2 previous errors
Build completed unsuccessfully in 0:03:40
