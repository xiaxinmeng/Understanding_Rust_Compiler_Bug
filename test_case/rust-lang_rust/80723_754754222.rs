plain
   Compiling rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
   Compiling rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
   Compiling rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
   Compiling rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error: call to noop method
   --> compiler/rustc_trait_selection/src/traits/error_reporting/mod.rs:831:38
    |
831 |                     .map(|arg| match arg.clone().kind {
    |
    |
    = note: `-D noop-method-call` implied by `-D warnings`
error: aborting due to previous error

error: could not compile `rustc_trait_selection`

