
error[E0495]: cannot infer an appropriate lifetime for lifetime parameter `'gcx` due to conflicting requirements                [62/861]
   --> librustc_typeck/check/mod.rs:629:17
    |
629 |             tcx.mk_region(ty::ReScope(region::Scope::CallSite(body.value.hir_id.local_id)))
    |                 ^^^^^^^^^
    |
note: first, the lifetime cannot outlive the lifetime 'gcx as defined on the impl at 622:1...
   --> librustc_typeck/check/mod.rs:622:1
    |
622 | impl<'a, 'gcx, 'tcx> Inherited<'a, 'gcx, 'tcx> {
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    = note: ...so that the types are compatible:
            expected rustc::ty::TyCtxt<'_, '_, '_>
               found rustc::ty::TyCtxt<'_, 'gcx, 'tcx>
    = note: but, the lifetime must be valid for the static lifetime...
    = note: ...so that the expression is assignable:
            expected std::boxed::Box<rustc::traits::TraitEngine<'_> + 'static>
               found std::boxed::Box<rustc::traits::TraitEngine<'_>>
error[E0495]: cannot infer an appropriate lifetime for lifetime parameter `'a` due to conflicting requirements
   --> librustc_typeck/coherence/builtin.rs:211:9
    |
211 |     tcx.infer_ctxt().enter(|infcx| {
    |         ^^^^^^^^^^
    |
note: first, the lifetime cannot outlive the lifetime 'a as defined on the function body at 175:1...
   --> librustc_typeck/coherence/builtin.rs:175:1
    |
175 | / pub fn coerce_unsized_info<'a, 'tcx>(tcx: TyCtxt<'a, 'tcx, 'tcx>,
176 | |                                      impl_did: DefId)
177 | |                                      -> CoerceUnsizedInfo {
178 | |     debug!("compute_coerce_unsized_info(impl_did={:?})", impl_did);                                                     [31/861]
...   |
404 | |     })
405 | | }
    | |_^
    = note: ...so that the types are compatible:
            expected rustc::ty::TyCtxt<'_, '_, '_>
               found rustc::ty::TyCtxt<'a, 'tcx, 'tcx>
note: but, the lifetime must be valid for the anonymous lifetime #2 defined on the body at 211:28...
   --> librustc_typeck/coherence/builtin.rs:211:28
    |
211 |       tcx.infer_ctxt().enter(|infcx| {
    |  ____________________________^
212 | |         let cause = ObligationCause::misc(span, impl_node_id);
213 | |         let check_mutbl = |mt_a: ty::TypeAndMut<'tcx>,
214 | |                            mt_b: ty::TypeAndMut<'tcx>,
...   |
403 | |         }
404 | |     })
    | |_____^
note: ...so that the declared lifetime parameter bounds are satisfied
   --> librustc_typeck/coherence/builtin.rs:228:17
    |
228 |                 check_mutbl(mt_a, mt_b, &|ty| tcx.mk_imm_ref(r_b, ty))
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
