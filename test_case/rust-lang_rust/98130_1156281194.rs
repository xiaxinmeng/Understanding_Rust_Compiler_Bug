plain
    Checking rustc_const_eval v0.0.0 (/checkout/compiler/rustc_const_eval)
    Checking rustc_ty_utils v0.0.0 (/checkout/compiler/rustc_ty_utils)
    Checking rustc_traits v0.0.0 (/checkout/compiler/rustc_traits)
    Checking rustc_mir_build v0.0.0 (/checkout/compiler/rustc_mir_build)
error[E0063]: missing field `local_var_defs` in initializer of `build::Builder<'_, '_>`
    |
834 |         let mut builder = Builder {
834 |         let mut builder = Builder {
    |                           ^^^^^^^ missing `local_var_defs`
error[E0061]: this function takes 5 arguments but 4 arguments were supplied
    --> compiler/rustc_mir_build/src/build/mod.rs:1003:27
     |
     |
1003 |             let pattern = pat_from_hir(tcx, self.param_env, self.typeck_results, pat);
     |                           ^^^^^^^^^^^^                                           --- an argument of type `&mut IndexVec<rustc_middle::thir::LocalVarId, rustc_middle::thir::LocalVarInfo>` is missing
note: function defined here
    --> compiler/rustc_mir_build/src/thir/pattern/mod.rs:49:15
     |
     |
49   | pub(crate) fn pat_from_hir<'a, 'tcx>(
50   |     tcx: TyCtxt<'tcx>,
     |     -----------------
     |     -----------------
51   |     param_env: ty::ParamEnv<'tcx>,
     |     -----------------------------
52   |     typeck_results: &'a ty::TypeckResults<'tcx>,
     |     -------------------------------------------
53   |     local_var_defs: &'a mut IndexVec<LocalVarId, LocalVarInfo>,
     |     ----------------------------------------------------------
54   |     pat: &'tcx hir::Pat<'tcx>,
help: provide the argument
     |
     |
1003 |             let pattern = pat_from_hir(tcx, self.param_env, self.typeck_results, {&mut IndexVec<rustc_middle::thir::LocalVarId, rustc_middle::thir::LocalVarInfo>}, pat);

error[E0609]: no field `0` on type `rustc_middle::thir::LocalVarId`
   --> compiler/rustc_mir_build/src/build/matches/mod.rs:714:74
    |
    |
714 |         if let Some(region_scope) = self.region_scope_tree.var_scope(var.0.local_id) && schedule_drop{

error[E0609]: no field `0` on type `rustc_middle::thir::LocalVarId`
   --> compiler/rustc_mir_build/src/build/matches/mod.rs:728:74
    |
    |
728 |         if let Some(region_scope) = self.region_scope_tree.var_scope(var.0.local_id) {

error: unreachable expression
   --> compiler/rustc_mir_build/src/thir/cx/expr.rs:909:13
    |
    |
909 |             ExprKind::UpvarRef { closure_def_id: self.body_owner, var_hir_id }
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unreachable expression
   ::: /checkout/library/core/src/panic.rs:57:9
    |
57  |         $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
    |         ---------------------------------------------------------------- any code following this expression is unreachable
    |         ---------------------------------------------------------------- any code following this expression is unreachable
    |
    = note: `-D unreachable-code` implied by `-D warnings`
error: unreachable expression
   --> compiler/rustc_mir_build/src/thir/cx/expr.rs:914:13
    |
    |
914 |             ExprKind::VarRef { id }
    |             ^^^^^^^^^^^^^^^^^^^^^^^ unreachable expression
   ::: /checkout/library/core/src/panic.rs:57:9
    |
57  |         $crate::panicking::panic_fmt($crate::const_format_args!($($t)+))
    |         ---------------------------------------------------------------- any code following this expression is unreachable
    |         ---------------------------------------------------------------- any code following this expression is unreachable

error[E0061]: this function takes 4 arguments but 3 arguments were supplied
   --> compiler/rustc_mir_build/src/thir/pattern/check_match.rs:137:25
    |
137 |         let mut patcx = PatCtxt::new(self.tcx, self.param_env, self.typeck_results);
    |                         ^^^^^^^^^^^^----------------------------------------------- an argument of type `&mut IndexVec<rustc_middle::thir::LocalVarId, rustc_middle::thir::LocalVarInfo>` is missing
note: associated function defined here
   --> compiler/rustc_mir_build/src/thir/pattern/mod.rs:67:19
    |
67  |     pub(crate) fn new(
67  |     pub(crate) fn new(
    |                   ^^^
68  |         tcx: TyCtxt<'tcx>,
    |         -----------------
69  |         param_env: ty::ParamEnv<'tcx>,
    |         -----------------------------
70  |         typeck_results: &'a ty::TypeckResults<'tcx>,
    |         -------------------------------------------
71  |         local_var_defs: &'a mut IndexVec<LocalVarId, LocalVarInfo>,
help: provide the argument
    |
    |
137 |         let mut patcx = PatCtxt::new(self.tcx, self.param_env, self.typeck_results, {&mut IndexVec<rustc_middle::thir::LocalVarId, rustc_middle::thir::LocalVarInfo>});

Some errors have detailed explanations: E0061, E0063, E0609.
For more information about an error, try `rustc --explain E0061`.
error: could not compile `rustc_mir_build` due to 7 previous errors
