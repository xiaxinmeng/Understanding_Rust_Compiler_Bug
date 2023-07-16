plain
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error: unused import: `valtree_to_const_value`
  --> compiler/rustc_const_eval/src/const_eval/mod.rs:25:45
   |
25 | pub(crate) use valtrees::{const_to_valtree, valtree_to_const_value};
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
    Checking rustc_typeck v0.0.0 (/checkout/compiler/rustc_typeck)
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
error: could not compile `rustc_const_eval` due to previous error
warning: build failed, waiting for other jobs to finish...
