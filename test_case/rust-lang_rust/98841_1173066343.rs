plain
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
    Checking rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error: unused import: `rustc_data_structures::fx::FxHashSet`
 --> compiler/rustc_passes/src/hir_id_validator.rs:1:5
  |
1 | use rustc_data_structures::fx::FxHashSet;
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`
error: could not compile `rustc_passes` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_passes` due to previous error
Build completed unsuccessfully in 0:03:14
