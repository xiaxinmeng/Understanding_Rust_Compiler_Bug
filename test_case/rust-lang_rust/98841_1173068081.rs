plain
   Compiling rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
   Compiling rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
   Compiling rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error: unused import: `rustc_data_structures::fx::FxHashSet`
 --> compiler/rustc_passes/src/hir_id_validator.rs:1:5
  |
1 | use rustc_data_structures::fx::FxHashSet;
  |
  |
  = note: `-D unused-imports` implied by `-D warnings`
[RUSTC-TIMING] rustc_passes test:false 1.310
error: could not compile `rustc_passes` due to previous error
warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] rustc_symbol_mangling test:false 6.341
