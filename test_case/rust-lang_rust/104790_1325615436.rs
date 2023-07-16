
error[E0599]: no method named `is_ty_uninhabited_from` found for struct `TyCtxt<'_>` in the current scope
   --> compiler/rustc_mir_build/src/errors.rs:396:28
    |
396 |             if self.cx.tcx.is_ty_uninhabited_from(self.cx.module, *sub_ty, self.cx.param_env) {
    |                            ^^^^^^^^^^^^^^^^^^^^^^ method not found in `TyCtxt<'_>`

