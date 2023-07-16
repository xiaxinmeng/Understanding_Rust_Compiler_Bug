plain
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error: unused variable: `handlers`
   --> compiler/rustc_codegen_llvm/src/back/write.rs:573:13
    |
573 |         let handlers = DiagnosticHandlers::new(cgcx, diag_handler, llcx);
    |             ^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_handlers`
    |
    = note: `-D unused-variables` implied by `-D warnings`
error: could not compile `rustc_codegen_llvm` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_codegen_llvm` due to previous error
Build completed unsuccessfully in 0:01:13
