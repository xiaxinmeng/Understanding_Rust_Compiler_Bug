plain
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
error[E0609]: no field `profile_sample_use` on type `CodegenOptions`
   --> compiler/rustc_codegen_ssa/src/back/write.rs:181:54
    |
181 |             pgo_sample_use: if_regular!(sess.opts.cg.profile_sample_use.clone(), None),
    |
    |
    = note: available fields are: `ar`, `code_model`, `codegen_units`, `control_flow_guard`, `debug_assertions` ... and 38 others
For more information about this error, try `rustc --explain E0609`.
error: could not compile `rustc_codegen_ssa` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
