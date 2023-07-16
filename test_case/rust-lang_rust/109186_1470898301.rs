plain
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
    Checking rustc_passes v0.0.0 (/checkout/compiler/rustc_passes)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error[E0599]: no method named `mk_fn_sig` found for struct `TyCtxt<'_>` in the current scope
     |
     |
2225 |         let expected_sig = tcx.mk_fn_sig(
     |                                ^^^^^^^^^ help: there is a method with a similar name: `fn_sig`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_passes` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_passes` due to previous error
