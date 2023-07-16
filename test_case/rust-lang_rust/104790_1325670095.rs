plain
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
error[E0599]: no method named `is_ty_uninhabited_from` found for struct `TyCtxt<'_>` in the current scope
   --> compiler/rustc_mir_build/src/errors.rs:396:28
    |
396 |             if self.cx.tcx.is_ty_uninhabited_from(self.cx.module, *sub_ty, self.cx.param_env) {

For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_mir_build` due to previous error
warning: build failed, waiting for other jobs to finish...
