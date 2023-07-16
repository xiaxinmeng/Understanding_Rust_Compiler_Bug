
--------------------------------------------------------------------------------
Files compared:   results/cgfilt-43c47db0b04627dbd0e16a1e7cad14a4a5a60d3a-keccak-Check-Full; results/cgfilt-fd426ee2bcc2183c3636c38927e5866d02146a13-keccak-Check-Full
Command:          /usr/local/rustup/toolchains/43c47db0b04627dbd0e16a1e7cad14a4a5a60d3a/bin/rustc --crate-name keccak src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata -C embed-bitcode=no -C debuginfo=2 -C metadata=7e4bc056058c3e31 -C extra-filename=-7e4bc056058c3e31 --out-dir /tmp/.tmpQs7GQY/target/debug/deps -L dependency=/tmp/.tmpQs7GQY/target/debug/deps -Adeprecated -Aunknown-lints -Zincremental-verify-ich; /usr/local/rustup/toolchains/fd426ee2bcc2183c3636c38927e5866d02146a13/bin/rustc --crate-name keccak src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata -C embed-bitcode=no -C debuginfo=2 -C metadata=7e4bc056058c3e31 -C extra-filename=-7e4bc056058c3e31 --out-dir /tmp/.tmpYRLPG5/target/debug/deps -L dependency=/tmp/.tmpYRLPG5/target/debug/deps -Adeprecated -Aunknown-lints -Zincremental-verify-ich
Data file:        results/cgfilt-diff-43c47db0b04627dbd0e16a1e7cad14a4a5a60d3a-fd426ee2bcc2183c3636c38927e5866d02146a13-keccak-Check-Full
Events recorded:  Ir
Events shown:     Ir
Event sort order: Ir
Thresholds:       0.1
Include dirs:     
User annotated:   
Auto-annotation:  on

--------------------------------------------------------------------------------
Ir          
--------------------------------------------------------------------------------
327,046,501  PROGRAM TOTALS

--------------------------------------------------------------------------------
Ir           file:function
--------------------------------------------------------------------------------
322,018,655  ???:<rustc_data_structures::obligation_forest::ObligationForest<rustc_trait_selection::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection::traits::fulfill::FulfillProcessor, rustc_data_structures::obligation_forest::Outcome<rustc_trait_selection::traits::fulfill::PendingPredicateObligation, rustc_infer::traits::FulfillmentErrorCode>>
 71,953,113  ???:<rustc_middle::ty::Ty as rustc_middle::ty::fold::TypeSuperFoldable>::super_fold_with::<rustc_infer::infer::resolve::OpportunisticVarResolver>
-66,339,597  ???:<rustc_infer::infer::resolve::OpportunisticVarResolver as rustc_middle::ty::fold::TypeFolder>::fold_ty
 16,425,516  ???:<rustc_infer::infer::InferCtxt>::commit_if_ok::<(), (), rustc_trait_selection::traits::project::assemble_candidates_from_impls::{closure
-14,072,523  ???:rustc_trait_selection::traits::project::opt_normalize_projection_type
-11,939,128  ???:<rustc_middle::ty::Predicate as rustc_middle::ty::fold::TypeFoldable>::has_type_flags
 11,939,128  ???:<rustc_middle::ty::Predicate as rustc_middle::ty::visit::TypeVisitable>::has_type_flags
 -9,706,118  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_infer::infer::resolve::OpportunisticVarResolver>
  6,686,712  ???:<rustc_infer::infer::ShallowResolver as rustc_middle::ty::fold::TypeFolder>::fold_ty
  6,262,100  ???:<rustc_infer::infer::resolve::OpportunisticVarResolver as rustc_middle::ty::fold::FallibleTypeFolder>::try_fold_ty
  5,455,662  ???:<rustc_privacy::DefIdVisitorSkeleton<rustc_privacy::TypePrivacyVisitor> as rustc_middle::ty::visit::TypeVisitor>::visit_ty
 -5,455,662  ???:<rustc_privacy::DefIdVisitorSkeleton<rustc_privacy::TypePrivacyVisitor> as rustc_middle::ty::fold::TypeVisitor>::visit_ty
  5,261,136  ???:<rustc_infer::infer::canonical::canonicalizer::Canonicalizer as rustc_middle::ty::fold::FallibleTypeFolder>::try_fold_predicate
 -4,513,722  ???:<rustc_middle::ty::consts::Const as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_middle::ty::fold::RegionFolder>
 -4,490,567  ???:<rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::query::evaluate_obligation::InferCtxtExt>::predicate_may_hold
  4,349,756  ???:<rustc_middle::ty::fold::RegionFolder as rustc_middle::ty::fold::FallibleTypeFolder>::try_fold_const
 -3,319,872  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::Ty> as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_infer::infer::resolve::OpportunisticVarResolver>
 -3,219,560  ???:<rustc_infer::infer::resolve::OpportunisticVarResolver as rustc_middle::ty::fold::FallibleTypeFolder>::try_fold_const
  2,892,198  ???:<rustc_middle::ty::PredicateKind as rustc_middle::ty::visit::TypeVisitable>::has_escaping_bound_vars
 -2,892,198  ???:<rustc_middle::ty::PredicateKind as rustc_middle::ty::fold::TypeFoldable>::has_escaping_bound_vars
  2,792,321  ???:<rustc_data_structures::snapshot_map::SnapshotMap<rustc_infer::traits::project::ProjectionCacheKey, rustc_infer::traits::project::ProjectionCacheEntry, &mut std::collections::hash::map::HashMap<rustc_infer::traits::project::ProjectionCacheKey, rustc_infer::traits::project::ProjectionCacheEntry, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, &mut rustc_infer::infer::undo_log::InferCtxtUndoLogs>>::insert
  2,753,996  ???:<rustc_infer::infer::InferCtxt>::probe::<core::result::Result<rustc_middle::traits::select::EvaluationResult, rustc_middle::traits::select::OverflowError>, rustc_trait_selection::traits::relationships::update<rustc_trait_selection::traits::fulfill::FulfillmentContext>::{closure
 -2,237,804  ???:<rustc_infer::traits::project::ProjectionCache>::ambiguous
 -2,094,401  ???:<rustc_trait_selection::traits::fulfill::FulfillmentContext as rustc_infer::traits::engine::TraitEngine>::register_predicate_obligation
  1,693,200  ???:<rustc_infer::infer::InferCtxt>::shallow_resolve::<rustc_middle::ty::consts::Const>
  1,533,556  ???:<rustc_middle::ty::Predicate as rustc_middle::ty::visit::TypeVisitable>::has_vars_bound_at_or_above
 -1,533,556  ???:<rustc_middle::ty::Predicate as rustc_middle::ty::fold::TypeFoldable>::has_vars_bound_at_or_above
  1,379,662  ???:<rustc_middle::ty::fold::RegionFolder as rustc_middle::ty::fold::FallibleTypeFolder>::try_fold_ty
 -1,228,990  ???:<rustc_middle::ty::Ty as rustc_middle::ty::fold::TypeSuperFoldable>::super_fold_with::<rustc_middle::ty::fold::RegionFolder>
 -1,012,901  ???:<rustc_borrowck::type_check::TypeChecker>::typeck_mir
    932,832  ???:<rustc_typeck::check::fn_ctxt::FnCtxt>::adjust_steps_as_infer_ok
   -917,540  ???:<hashbrown::raw::RawTable<(rustc_middle::ty::context::InternedInSet<rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>>, ())>>::reserve_rehash::<hashbrown::map::make_hasher<rustc_middle::ty::context::InternedInSet<rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>>, rustc_middle::ty::context::InternedInSet<rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>>, (), core::hash::BuildHasherDefault<rustc_hash::FxHasher>>::{closure
   -863,158  ???:<rustc_middle::ty::consts::Const as rustc_middle::ty::fold::TypeFoldable>::visit_with::<rustc_privacy::DefIdVisitorSkeleton<rustc_privacy::TypePrivacyVisitor>>
    863,158  ???:<rustc_middle::ty::consts::Const as rustc_middle::ty::visit::TypeVisitable>::visit_with::<rustc_privacy::DefIdVisitorSkeleton<rustc_privacy::TypePrivacyVisitor>>
   -859,237  ???:<rustc_borrowck::type_check::TypeVerifier as rustc_middle::mir::visit::Visitor>::visit_body
   -749,616  ???:<rustc_infer::infer::canonical::canonicalizer::Canonicalizer>::canonicalize::<rustc_middle::ty::ParamEnvAnd<rustc_middle::ty::Predicate>>
   -716,961  ???:<rustc_trait_selection::traits::fulfill::FulfillProcessor>::process_changed_obligations
   -708,192  ???:<alloc::vec::Vec<rustc_middle::ty::adjustment::Adjustment> as alloc::vec::spec_from_iter::SpecFromIter<rustc_middle::ty::adjustment::Adjustment, core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::map::Map<core::slice::iter::Iter<(rustc_middle::ty::Ty, rustc_trait_selection::autoderef::AutoderefKind)>, <rustc_typeck::check::fn_ctxt::FnCtxt>::adjust_steps_as_infer_ok::{closure
   -675,176  ???:<rustc_middle::ty::Ty as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_infer::infer::resolve::OpportunisticVarResolver>
   -619,248  ???:<rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_kind
    619,152  ???:<rustc_infer::infer::InferCtxt as rustc_trait_selection::infer::InferCtxtExt>::partially_normalize_associated_types_in::<rustc_middle::ty::sty::FnSig>
    618,428  ???:<rustc_mir_build::build::CFG>::push_assign_unit
    584,196  ???:<rustc_borrowck::borrow_set::BorrowSet>::build
   -572,468  ???:<rustc_middle::ty::consts::Const as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_infer::infer::resolve::OpportunisticVarResolver>
   -467,868  ???:rustc_data_structures::graph::dominators::dominators::<&rustc_middle::mir::Body>
   -457,124  ???:<rustc_mir_build::build::Builder>::ast_block_stmts
    418,704  ???:rustc_middle::ty::util::fold_list::<rustc_trait_selection::traits::project::AssocTypeNormalizer, rustc_middle::ty::Ty, <&rustc_middle::ty::list::List<rustc_middle::ty::Ty> as rustc_middle::ty::fold::TypeFoldable>::try_fold_with<rustc_trait_selection::traits::project::AssocTypeNormalizer>::{closure
    418,198  ???:<rustc_middle::ty::context::CtxtInterners>::intern_predicate
    401,184  ???:<rustc_middle::ty::sty::EarlyBinder<rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::FnSig>> as rustc_middle::ty::subst::Subst>::subst
    396,704  ???:<rustc_middle::ty::Ty as rustc_middle::ty::visit::TypeVisitable>::visit_with::<rustc_privacy::DefIdVisitorSkeleton<rustc_privacy::TypePrivacyVisitor>>
   -396,704  ???:<rustc_middle::ty::Ty as rustc_middle::ty::fold::TypeFoldable>::visit_with::<rustc_privacy::DefIdVisitorSkeleton<rustc_privacy::TypePrivacyVisitor>>
    392,904  ???:<rustc_infer::infer::InferCtxt>::replace_bound_vars_with_fresh_vars::<rustc_middle::ty::sty::FnSig>
    387,849  ???:<alloc::vec::Vec<(rustc_ast::tokenstream::TokenTree, rustc_ast::tokenstream::Spacing)> as core::ops::drop::Drop>::drop
   -376,916  ???:<alloc::rc::Rc<alloc::vec::Vec<(rustc_ast::tokenstream::TokenTree, rustc_ast::tokenstream::Spacing)>> as core::ops::drop::Drop>::drop
   -367,642  ???:<rustc_data_structures::obligation_forest::ObligationForest<rustc_trait_selection::traits::fulfill::PendingPredicateObligation>>::apply_rewrites
   -366,333  ???:_rjem_je_eset_fit
    344,416  ???:<rustc_middle::ty::Ty as rustc_middle::ty::visit::TypeVisitable>::visit_with::<<rustc_middle::ty::context::TyCtxt>::any_free_region_meets::RegionVisitor<<rustc_middle::ty::context::TyCtxt>::all_free_regions_meet<rustc_middle::ty::Ty, rustc_borrowck::type_check::liveness::compute_live_locals::{closure
   -344,416  ???:<rustc_middle::ty::Ty as rustc_middle::ty::fold::TypeFoldable>::visit_with::<<rustc_middle::ty::context::TyCtxt>::any_free_region_meets::RegionVisitor<<rustc_middle::ty::context::TyCtxt>::all_free_regions_meet<rustc_middle::ty::Ty, rustc_borrowck::type_check::liveness::compute_live_locals::{closure
   -328,320  ???:<rustc_middle::ty::sty::ProjectionTy as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_infer::infer::resolve::OpportunisticVarResolver>
