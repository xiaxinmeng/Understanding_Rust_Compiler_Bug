
error[E0425]: cannot find function `LLVMGetTarget` in module `llvm`    ] 17/18: rustc_codegen_llvm
   --> src\librustc_codegen_llvm\back\write.rs:801:24
    |
801 |     let target = llvm::LLVMGetTarget(llmod);
    |                        ^^^^^^^^^^^^^ help: a function with a similar name exists: `LLVMGetParam`

error: aborting due to previous error
