plain
    Checking rustc_const_eval v0.0.0 (/checkout/compiler/rustc_const_eval)
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error[E0423]: expected function, tuple struct or tuple variant, found struct `ops::FnCallNonConst`
   --> compiler/rustc_const_eval/src/transform/check_consts/check.rs:834:47
    |
834 |   ...                   self.check_op(ops::FnCallNonConst(None));
    |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^ help: use struct literal syntax instead: `ops::FnCallNonConst { caller: val, callee: val, substs: val, span: val, from_hir_call: val }`
   ::: compiler/rustc_const_eval/src/transform/check_consts/ops.rs:85:1
    |
    |
85  | / pub struct FnCallNonConst<'tcx> {
86  | |     pub caller: DefId,
87  | |     pub callee: DefId,
88  | |     pub substs: SubstsRef<'tcx>,
89  | |     pub span: Span,
90  | |     pub from_hir_call: bool,
91  | | }
    | |_- `ops::FnCallNonConst` defined here
For more information about this error, try `rustc --explain E0423`.
error: could not compile `rustc_const_eval` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
