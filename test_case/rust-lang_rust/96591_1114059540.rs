plain
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error: unused imports: `Allocation`, `ConstValue`
 --> compiler/rustc_mir_build/src/thir/constant.rs:5:5
  |
5 |     Allocation, ConstValue, LitToConstError, LitToConstInput, Scalar,
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`
error: unused import: `rustc_errors::ErrorGuaranteed`
 --> compiler/rustc_const_eval/src/const_eval/mod.rs:5:5
  |
5 | use rustc_errors::ErrorGuaranteed;
5 | use rustc_errors::ErrorGuaranteed;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `-D unused-imports` implied by `-D warnings`
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error: could not compile `rustc_mir_build` due to previous error
warning: build failed, waiting for other jobs to finish...
