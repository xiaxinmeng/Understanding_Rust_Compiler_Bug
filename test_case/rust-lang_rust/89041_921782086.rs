plain
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error[E0425]: cannot find value `cu1` in this scope
   --> compiler/rustc_codegen_llvm/src/back/lto.rs:334:65
    |
334 |             unsafe { llvm::LLVMRustLTOPatchDICompileUnit(llmod, cu1) };

For more information about this error, try `rustc --explain E0425`.
error: could not compile `rustc_codegen_llvm` due to previous error
warning: build failed, waiting for other jobs to finish...
