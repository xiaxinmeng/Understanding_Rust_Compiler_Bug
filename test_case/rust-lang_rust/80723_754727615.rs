plain
   Compiling rustc_symbol_mangling v0.0.0 (/checkout/compiler/rustc_symbol_mangling)
   Compiling rustc_incremental v0.0.0 (/checkout/compiler/rustc_incremental)
   Compiling rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
   Compiling rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
error: call to noop method
  --> compiler/rustc_codegen_ssa/src/back/rpath.rs:27:16
   |
27 |     let libs = config.used_crates.clone();
   |
   |
   = note: `-D noop-method-call` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `rustc_codegen_ssa`

