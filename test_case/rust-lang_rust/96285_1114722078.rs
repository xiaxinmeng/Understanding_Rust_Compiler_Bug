plain
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error[E0609]: no field `virtual_function_elimination` on type `CodegenOptions`
  --> compiler/rustc_codegen_ssa/src/meth.rs:28:35
   |
28 |         if bx.cx().sess().opts.cg.virtual_function_elimination && bx.cx().sess().lto() == Lto::Fat {
   |
   |
   = note: available fields are: `ar`, `code_model`, `codegen_units`, `control_flow_guard`, `debug_assertions` ... and 41 others
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
For more information about this error, try `rustc --explain E0609`.
error: could not compile `rustc_codegen_ssa` due to previous error
warning: build failed, waiting for other jobs to finish...
