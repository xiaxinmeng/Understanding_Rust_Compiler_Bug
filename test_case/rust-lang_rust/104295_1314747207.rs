plain
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error: variable does not need to be mutable
   --> compiler/rustc_hir_analysis/src/check/compare_method.rs:159:9
    |
159 |     let mut cause = ObligationCause::new(
    |         |
    |         help: remove this `mut`
    |
    |
    = note: `-D unused-mut` implied by `-D warnings`
error: variable does not need to be mutable
   --> compiler/rustc_hir_analysis/src/check/compare_method.rs:360:9
    |
    |
360 |     let mut cause = ObligationCause::new(
    |         |
    |         help: remove this `mut`

    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
