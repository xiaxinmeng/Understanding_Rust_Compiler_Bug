plain

error: cannot determine resolution for the macro `cstr`
   --> compiler/rustc_codegen_llvm/src/base.rs:113:46
    |
113 |                 cx.create_used_variable_impl(cstr!("llvm.used"), &*cx.used_statics.borrow());
    |
    |
    = note: import resolution is stuck, try simplifying macro imports
error: cannot determine resolution for the macro `cstr`
   --> compiler/rustc_codegen_llvm/src/base.rs:117:21
    |
    |
117 |                     cstr!("llvm.compiler.used"),
    |
    |
    = note: import resolution is stuck, try simplifying macro imports
For more information about this error, try `rustc --explain E0432`.
error: could not compile `rustc_codegen_llvm` due to 3 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_codegen_llvm` due to 3 previous errors
