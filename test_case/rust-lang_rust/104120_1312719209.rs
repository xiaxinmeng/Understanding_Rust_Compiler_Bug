plain
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
    Checking rustc_hir_typeck v0.1.0 (/checkout/compiler/rustc_hir_typeck)
    Checking rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
error: diagnostic slug and crate name do not match
    |
    |
130 | #[diag(hir_analysis_op_trait_generic_params)]
    |
    |
    = note: slug is `hir_analysis_op_trait_generic_params` but the crate name is `rustc_hir_typeck`
    = help: expected a slug starting with `hir_typeck_...`
error: could not compile `rustc_hir_typeck` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_hir_typeck` due to previous error
Build completed unsuccessfully in 0:03:34
