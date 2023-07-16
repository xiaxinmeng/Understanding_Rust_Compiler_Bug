plain
[RUSTC-TIMING] rustc_const_eval test:true 3.438
[RUSTC-TIMING] rustc_const_eval test:false 3.471
    Checking rustc_mir_transform v0.0.0 (/checkout/compiler/rustc_mir_transform)
    Checking rustc_borrowck v0.0.0 (/checkout/compiler/rustc_borrowck)
error[E0599]: the method `subst_identity` exists for struct `EarlyBinder<EarlyBinder<Ty<'_>>>`, but its trait bounds were not satisfied
   --> compiler/rustc_hir_analysis/src/collect/predicates_of.rs:182:84
    |
182 |                 let ct_ty = ty::EarlyBinder(tcx.type_of(param.def_id.to_def_id())).subst_identity();
    |                                                                                    ^^^^^^^^^^^^^^ method cannot be called on `EarlyBinder<EarlyBinder<Ty<'_>>>` due to unsatisfied trait bounds
   ::: /checkout/compiler/rustc_middle/src/ty/subst.rs:555:1
    |
    |
555 | pub struct EarlyBinder<T>(pub T);
    | |
    | |
    | doesn't satisfy `_: TypeFoldable<TyCtxt<'_>>`
    | doesn't satisfy `_: TypeVisitable<TyCtxt<'_>>`
    | doesn't satisfy `_: TypeVisitableExt<'_>`
    | doesn't satisfy `_: rustc_middle::ty::TypeFoldable<'_>`
    | doesn't satisfy `_: rustc_middle::ty::TypeVisitable<'_>`
    = note: the following trait bounds were not satisfied:
    = note: the following trait bounds were not satisfied:
            `EarlyBinder<rustc_middle::ty::Ty<'_>>: rustc_middle::ty::TypeFoldable<'_>`
            `EarlyBinder<rustc_middle::ty::Ty<'_>>: rustc_middle::ty::TypeVisitable<'_>`
            which is required by `EarlyBinder<rustc_middle::ty::Ty<'_>>: rustc_middle::ty::TypeFoldable<'_>`
            `EarlyBinder<rustc_middle::ty::Ty<'_>>: TypeVisitableExt<'_>`
            which is required by `EarlyBinder<rustc_middle::ty::Ty<'_>>: rustc_middle::ty::TypeFoldable<'_>`
            `EarlyBinder<rustc_middle::ty::Ty<'_>>: rustc_type_ir::visit::TypeVisitable<TyCtxt<'_>>`
            which is required by `EarlyBinder<rustc_middle::ty::Ty<'_>>: rustc_middle::ty::TypeFoldable<'_>`
            `EarlyBinder<rustc_middle::ty::Ty<'_>>: rustc_type_ir::fold::TypeFoldable<TyCtxt<'_>>`
            which is required by `EarlyBinder<rustc_middle::ty::Ty<'_>>: rustc_middle::ty::TypeFoldable<'_>`
For more information about this error, try `rustc --explain E0599`.
[RUSTC-TIMING] rustc_hir_analysis test:true 2.430
error: could not compile `rustc_hir_analysis` due to previous error
warning: build failed, waiting for other jobs to finish...
