plain
    Checking rustc_plugin_impl v0.0.0 (/checkout/compiler/rustc_plugin_impl)
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
error[E0599]: no method named `add_implied_bounds` found for struct `OutlivesEnvironment` in the current scope
   --> compiler/rustc_typeck/src/impl_wf_check/min_specialization.rs:156:18
156 |     outlives_env.add_implied_bounds(
156 |     outlives_env.add_implied_bounds(
    |                  ^^^^^^^^^^^^^^^^^^ method not found in `OutlivesEnvironment<'_>`
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error: unused import: `crate::check::regionck::OutlivesEnvironmentExt`
  --> compiler/rustc_typeck/src/impl_wf_check/min_specialization.rs:68:5
   |
68 | use crate::check::regionck::OutlivesEnvironmentExt;
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_typeck` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_typeck` due to 2 previous errors
