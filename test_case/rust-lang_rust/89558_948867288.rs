plain
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
    Checking rustc_const_eval v0.0.0 (/checkout/compiler/rustc_const_eval)
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error[E0609]: no field `actually_rustdoc` on type `&Session`
   |
   |
73 |         if cx.tcx.sess.actually_rustdoc {
   |
   = note: available fields are: `target`, `host`, `opts`, `host_tlib_path`, `target_tlib_path` ... and 15 others
help: one of the expressions' fields has a field of the same name
   |
   |
73 |         if cx.tcx.sess.opts.actually_rustdoc {

For more information about this error, try `rustc --explain E0609`.
error: could not compile `rustc_lint` due to previous error
warning: build failed, waiting for other jobs to finish...
