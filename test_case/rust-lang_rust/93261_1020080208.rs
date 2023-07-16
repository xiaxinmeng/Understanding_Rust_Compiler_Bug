plain
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error[E0425]: cannot find value `catch_switch` in this scope
     |
     |
1033 |                 llvm::LLVMRustAddHandler(catch_switch, handler);
     |                                          ^^^^^^^^^^^^ help: you might have meant to call the method: `self.catch_switch`
For more information about this error, try `rustc --explain E0425`.
error: could not compile `rustc_codegen_llvm` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
