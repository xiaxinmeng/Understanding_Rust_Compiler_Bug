plain
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
error[E0615]: attempted to take value of method `index` on type `TyVid`
   --> compiler/rustc_infer/src/infer/type_variable.rs:199:37
    |
199 |         self.storage.values.get(vid.index as usize).diverging
    |
help: use parentheses to call the method
    |
    |
199 |         self.storage.values.get(vid.index() as usize).diverging

For more information about this error, try `rustc --explain E0615`.
error: could not compile `rustc_infer` due to previous error
warning: build failed, waiting for other jobs to finish...
