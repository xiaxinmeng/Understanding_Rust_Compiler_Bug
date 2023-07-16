plain
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking rustc_save_analysis v0.0.0 (/checkout/compiler/rustc_save_analysis)
error[E0609]: no field `fn_align` on type `&Features`
     |
     |
1570 |                     if let (Target::Fn, false) = (target, self.tcx.features().fn_align) {
     |
     |
     = note: available fields are: `declared_lang_features`, `declared_lib_features`, `active_features`, `abi_thiscall`, `abi_unadjusted` ... and 176 others
For more information about this error, try `rustc --explain E0609`.
error: could not compile `rustc_passes` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_passes` due to previous error
