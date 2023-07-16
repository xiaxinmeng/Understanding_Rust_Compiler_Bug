plain
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
    Checking rustc_smir v0.0.0 (/checkout/compiler/rustc_smir)
error[E0317]: `if` may be missing an `else` clause
    --> compiler/rustc_metadata/src/rmeta/encoder.rs:1049:21
     |
1049 | /                     if tcx.opt_rpitit_info(def_id.to_def_id()).is_some() {
     | |                         ---- found here
1051 | |                     }
     | |_____________________^ expected `bool`, found `()`
     |
     |
     = note: `if` expressions without `else` evaluate to `()`
     = help: consider adding an `else` block that evaluates to the expected type
For more information about this error, try `rustc --explain E0317`.
error: could not compile `rustc_metadata` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_metadata` due to previous error
