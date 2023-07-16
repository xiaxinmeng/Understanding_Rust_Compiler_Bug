plain
    Checking rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
    Checking rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
    Checking rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
    Checking rustc_smir v0.0.0 (/checkout/compiler/rustc_smir)
error[E0599]: no method named `mk_ty_var` found for struct `TyCtxt<'tcx>` in the current scope
   --> compiler/rustc_infer/src/infer/canonical/canonicalizer.rs:385:40
    |
385 |                     t = self.infcx.tcx.mk_ty_var(root_vid);

    Checking rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
    Checking rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
error[E0599]: no method named `mk_const` found for struct `TyCtxt<'tcx>` in the current scope
error[E0599]: no method named `mk_const` found for struct `TyCtxt<'tcx>` in the current scope
   --> compiler/rustc_infer/src/infer/canonical/canonicalizer.rs:500:41
    |
500 |                     ct = self.infcx.tcx.mk_const(ty::InferConst::Var(root_vid), ct.ty());
    |                                         ^^^^^^^^ help: there is a method with a similar name: `mir_const`

error[E0599]: no method named `mk_int_var` found for struct `TyCtxt<'tcx>` in the current scope
     |
     |
1370 |             self.tcx.mk_int_var(inner.int_unification_table().find(vid))


error[E0599]: no method named `mk_float_var` found for struct `TyCtxt<'tcx>` in the current scope
     |
     |
1381 |             self.tcx.mk_float_var(inner.float_unification_table().find(vid))

For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_infer` due to 4 previous errors
warning: build failed, waiting for other jobs to finish...
