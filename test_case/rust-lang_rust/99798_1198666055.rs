plain
    |
451 | pub fn const_expr<'tcx>(
    |        ^^^^^^^^^^
    |
    = note: `-D dead-code` implied by `-D warnings`
error: function `recurse_build` is never used
   --> compiler/rustc_ty_utils/src/consts.rs:471:4
    |
471 | fn recurse_build<'tcx>(
---

error: function `error` is never used
   --> compiler/rustc_ty_utils/src/consts.rs:689:4
    |
689 | fn error<'tcx>(tcx: TyCtxt<'tcx>, span: Span, msg: &str) -> Result<!, ErrorGuaranteed> {

error: could not compile `rustc_ty_utils` due to 4 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_ty_utils` due to 4 previous errors
