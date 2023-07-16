plain
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error[E0425]: cannot find value `pac` in this scope
    |
    |
286 |                 pac.is_some().into(),


error[E0425]: cannot find value `pac` in this scope
    |
    |
288 |             let pac_opts = pac.unwrap_or(PacRet { leaf: false, key: PAuthKey::A });

For more information about this error, try `rustc --explain E0425`.
error: could not compile `rustc_codegen_llvm` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
