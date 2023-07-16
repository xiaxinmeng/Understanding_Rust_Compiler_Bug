plain
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
    Checking rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
    Checking rustc_privacy v0.0.0 (/checkout/compiler/rustc_privacy)
    Checking rustc_hir_typeck v0.1.0 (/checkout/compiler/rustc_hir_typeck)
error[E0599]: no method named `normalize_associated_types_in` found for reference `&fn_ctxt::FnCtxt<'a, 'tcx>` in the current scope
    --> compiler/rustc_hir_typeck/src/fn_ctxt/_impl.rs:1281:35
     |
1281 |         let ty_substituted = self.normalize_associated_types_in(span, ty.subst(tcx, substs));
     |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `&fn_ctxt::FnCtxt<'a, 'tcx>`

error[E0599]: no method named `normalize_associated_types_in` found for reference `&fn_ctxt::FnCtxt<'a, 'tcx>` in the current scope
    --> compiler/rustc_hir_typeck/src/fn_ctxt/_impl.rs:1289:32
     |
1289 |             let impl_ty = self.normalize_associated_types_in(
     |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `&fn_ctxt::FnCtxt<'a, 'tcx>`
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustc_hir_typeck` due to 2 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_hir_typeck` due to 2 previous errors
