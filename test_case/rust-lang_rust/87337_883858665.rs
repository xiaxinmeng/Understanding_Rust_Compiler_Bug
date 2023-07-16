plain
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error[E0423]: expected value, found struct variant `Level::Error`
    --> compiler/rustc_codegen_llvm/src/back/write.rs:286:41
     |
286  |         llvm::DiagnosticLevel::Error => Level::Error,
     | 
    ::: /checkout/compiler/rustc_errors/src/lib.rs:1106:5
     |
1106 |     Error {
1106 |     Error {
     |     ----- `Level::Error` defined here
help: use struct literal syntax instead
     |
     |
286  |         llvm::DiagnosticLevel::Error => Level::Error { /* fields */ },
help: consider importing one of these items instead
     |
1    | use core::fmt::Error;
     |
     |
1    | use crate::llvm::DiagnosticLevel::Error;
1    | use rustc_data_structures::obligation_forest::ProcessResult::Error;
     |
1    | use rustc_errors::DiagnosticId::Error;
     |
