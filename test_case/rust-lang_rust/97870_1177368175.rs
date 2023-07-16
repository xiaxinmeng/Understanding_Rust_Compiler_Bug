none
--------------------------------------------------------------------------------
Files compared:   results/cgfilt-7425fb293f510a6f138e82a963a3bc599a5b9e1c-cranelift-codegen-0.82.1-Check-Full; results/cgfilt-72423f087b612b625133cd04348ee1bc3d887e43-cranelift-codegen-0.82.1-Check-Full
Command:          /usr/local/rustup/toolchains/7425fb293f510a6f138e82a963a3bc599a5b9e1c/bin/rustc --crate-name cranelift_codegen --edition=2018 src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata -C embed-bitcode=no -C debuginfo=2 --cfg feature="default" --cfg feature="gimli" --cfg feature="std" --cfg feature="unwind" -C metadata=0cdcc3646ceb2365 -C extra-filename=-0cdcc3646ceb2365 --out-dir /tmp/.tmpE2qgrH/target/debug/deps -L dependency=/tmp/.tmpE2qgrH/target/debug/deps --extern cranelift_bforest=/tmp/.tmpE2qgrH/target/debug/deps/libcranelift_bforest-b49c6be2343154ae.rmeta --extern cranelift_codegen_shared=/tmp/.tmpE2qgrH/target/debug/deps/libcranelift_codegen_shared-2571f8f2275f7605.rmeta --extern cranelift_entity=/tmp/.tmpE2qgrH/target/debug/deps/libcranelift_entity-c391de30ba436e90.rmeta --extern gimli=/tmp/.tmpE2qgrH/target/debug/deps/libgimli-1d637e55d6b50218.rmeta --extern log=/tmp/.tmpE2qgrH/target/debug/deps/liblog-e724a7b6173dd03e.rmeta --extern regalloc=/tmp/.tmpE2qgrH/target/debug/deps/libregalloc-60dbf5b5aa455e72.rmeta --extern smallvec=/tmp/.tmpE2qgrH/target/debug/deps/libsmallvec-61e548ea0cf3be6d.rmeta --extern target_lexicon=/tmp/.tmpE2qgrH/target/debug/deps/libtarget_lexicon-57e59898b1f55589.rmeta --cfg feature="x86" -Adeprecated -Aunknown-lints -Zincremental-verify-ich; /usr/local/rustup/toolchains/72423f087b612b625133cd04348ee1bc3d887e43/bin/rustc --crate-name cranelift_codegen --edition=2018 src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata -C embed-bitcode=no -C debuginfo=2 --cfg feature="default" --cfg feature="gimli" --cfg feature="std" --cfg feature="unwind" -C metadata=0cdcc3646ceb2365 -C extra-filename=-0cdcc3646ceb2365 --out-dir /tmp/.tmpEDp8Z4/target/debug/deps -L dependency=/tmp/.tmpEDp8Z4/target/debug/deps --extern cranelift_bforest=/tmp/.tmpEDp8Z4/target/debug/deps/libcranelift_bforest-b49c6be2343154ae.rmeta --extern cranelift_codegen_shared=/tmp/.tmpEDp8Z4/target/debug/deps/libcranelift_codegen_shared-2571f8f2275f7605.rmeta --extern cranelift_entity=/tmp/.tmpEDp8Z4/target/debug/deps/libcranelift_entity-c391de30ba436e90.rmeta --extern gimli=/tmp/.tmpEDp8Z4/target/debug/deps/libgimli-1d637e55d6b50218.rmeta --extern log=/tmp/.tmpEDp8Z4/target/debug/deps/liblog-e724a7b6173dd03e.rmeta --extern regalloc=/tmp/.tmpEDp8Z4/target/debug/deps/libregalloc-60dbf5b5aa455e72.rmeta --extern smallvec=/tmp/.tmpEDp8Z4/target/debug/deps/libsmallvec-61e548ea0cf3be6d.rmeta --extern target_lexicon=/tmp/.tmpEDp8Z4/target/debug/deps/libtarget_lexicon-57e59898b1f55589.rmeta --cfg feature="x86" -Adeprecated -Aunknown-lints -Zincremental-verify-ich
Data file:        results/cgfilt-diff-7425fb293f510a6f138e82a963a3bc599a5b9e1c-72423f087b612b625133cd04348ee1bc3d887e43-cranelift-codegen-0.82.1-Check-Full
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
215,143,446  PROGRAM TOTALS

--------------------------------------------------------------------------------
Ir           file:function
--------------------------------------------------------------------------------
233,015,777  ???:<rustc_data_structures::obligation_forest::ObligationForest<rustc_trait_selection::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection::traits::fulfill::FulfillProcessor, rustc_data_structures::obligation_forest::Outcome<rustc_trait_selection::traits::fulfill::PendingPredicateObligation, rustc_infer::traits::FulfillmentErrorCode>>
-24,410,765  ???:<rustc_trait_selection::traits::wf::WfPredicates>::nominal_obligations
 21,424,454  ???:<rustc_trait_selection::traits::wf::WfPredicates>::compute
-12,963,890  ???:<alloc::rc::Rc<rustc_middle::traits::ObligationCauseCode> as core::ops::drop::Drop>::drop
-10,684,303  ???:<rustc_typeck::check::method::probe::ProbeContext>::assemble_inherent_impl_probe
  9,504,383  ???:<rustc_typeck::check::method::probe::ProbeContext>::assemble_inherent_impl_candidates_for_type
 -6,451,979  ???:<rustc_mir_transform::function_item_references::FunctionItemReferences as rustc_mir_transform::pass_manager::MirLint>::run_lint
  4,920,108  ???:<<rustc_middle::ty::context::TyCtxt>::any_free_region_meets::RegionVisitor<<rustc_middle::ty::context::TyCtxt>::for_each_free_region<rustc_middle::ty::Ty, <rustc_borrowck::type_check::liveness::trace::LivenessContext>::make_all_regions_live<rustc_middle::ty::Ty>::{closure
  4,744,445  ???:core::ptr::drop_in_place::<rustc_middle::traits::ObligationCauseCode>
 -4,592,024  ???:rustc_trait_selection::traits::project::opt_normalize_projection_type
 -4,512,951  ???:<rustc_middle::ty::sty::Region as rustc_middle::ty::fold::TypeFoldable>::visit_with::<<rustc_middle::ty::context::TyCtxt>::any_free_region_meets::RegionVisitor<<rustc_middle::ty::context::TyCtxt>::for_each_free_region<rustc_middle::ty::Ty, <rustc_borrowck::type_check::liveness::trace::LivenessContext>::make_all_regions_live<rustc_middle::ty::Ty>::{closure
  2,918,224  ???:<rustc_infer::infer::InferCtxt>::commit_if_ok::<(), (), rustc_trait_selection::traits::project::assemble_candidates_from_impls::{closure
  2,419,076  ???:<alloc::vec::drain_filter::DrainFilter<rustc_infer::traits::Obligation<rustc_middle::ty::Predicate>, rustc_trait_selection::traits::project::opt_normalize_projection_type::{closure
 -1,987,571  ???:rustc_middle::ty::util::fold_list::<rustc_middle::ty::fold::BoundVarReplacer, rustc_middle::ty::Ty, <&rustc_middle::ty::list::List<rustc_middle::ty::Ty> as rustc_middle::ty::fold::TypeFoldable>::try_fold_with<rustc_middle::ty::fold::BoundVarReplacer>::{closure
  1,847,278  ???:<rustc_data_structures::obligation_forest::ObligationForest<rustc_trait_selection::traits::fulfill::PendingPredicateObligation>>::compress::<<rustc_data_structures::obligation_forest::ObligationForest<rustc_trait_selection::traits::fulfill::PendingPredicateObligation>>::process_obligations<rustc_trait_selection::traits::fulfill::FulfillProcessor, rustc_data_structures::obligation_forest::Outcome<rustc_trait_selection::traits::fulfill::PendingPredicateObligation, rustc_infer::traits::FulfillmentErrorCode>>::{closure
  1,480,296  ???:<rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>>::for_item::<<rustc_typeck::check::method::probe::ProbeContext>::fresh_item_substs::{closure
  1,455,896  ???:<rustc_borrowck::type_check::TypeChecker>::typeck_mir
 -1,314,000  ???:<rustc_middle::mir::BasicBlockData>::expand_statements::<<rustc_mir_transform::deaggregator::Deaggregator as rustc_middle::mir::MirPass>::run_pass::{closure
  1,310,339  ???:<rustc_mir_transform::deaggregator::Deaggregator as rustc_middle::mir::MirPass>::run_pass
 -1,189,433  ???:<rustc_borrowck::MirBorrowckCtxt as rustc_mir_dataflow::framework::visitor::ResultsVisitor>::visit_statement_before_primary_effect
 -1,046,329  ???:rustc_mir_transform::check_unsafety::unsafety_check_result
 -1,028,421  ???:<rustc_middle::ty::Ty as rustc_middle::ty::fold::TypeSuperFoldable>::super_visit_with::<<rustc_middle::ty::context::TyCtxt>::any_free_region_meets::RegionVisitor<<rustc_middle::ty::context::TyCtxt>::for_each_free_region<rustc_middle::ty::Ty, <rustc_borrowck::type_check::liveness::trace::LivenessContext>::make_all_regions_live<rustc_middle::ty::Ty>::{closure
   -992,447  ???:_rjem_je_arena_cache_bin_fill_small
   -965,746  ???:<rustc_typeck::check::method::probe::ProbeContext>::assemble_inherent_candidates
    948,985  ???:<(rustc_middle::ty::sty::FnSig, rustc_middle::ty::InstantiatedPredicates) as rustc_middle::ty::fold::TypeFoldable>::fold_with::<rustc_infer::infer::resolve::OpportunisticVarResolver>
   -948,862  ???:<rustc_infer::infer::InferCtxt>::instantiate_nll_query_response_and_region_obligations::<()>
    877,569  ???:<rustc_typeck::check::method::probe::ProbeContext>::assemble_inherent_candidates_for_incoherent_ty
    792,219  ???:<rustc_middle::hir::map::Map>::local_def_id_to_hir_id
   -669,169  ???:<rustc_typeck::check::writeback::WritebackCx as rustc_hir::intravisit::Visitor>::visit_expr
   -586,837  ???:rustc_trait_selection::traits::project::normalize::<(rustc_middle::ty::sty::FnSig, rustc_middle::ty::InstantiatedPredicates)>
    582,085  ???:core::ptr::drop_in_place::<rustc_infer::infer::InferCtxtBuilder>
   -559,957  ???:core::ptr::drop_in_place::<rustc_typeck::check::inherited::InheritedBuilder>
    500,205  ???:<rustc_infer::infer::InferCtxt as rustc_trait_selection::infer::InferCtxtExt>::partially_normalize_associated_types_in::<(rustc_middle::ty::sty::FnSig, rustc_middle::ty::InstantiatedPredicates)>
    488,343  ???:<rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>>::for_item::<rustc_typeck::check::wfcheck::check_where_clauses::{closure
   -479,072  ???:_rjem_je_malloc_default
    468,475  ???:<rustc_trait_selection::traits::select::SelectionContext>::select
   -458,620  ???:<hashbrown::raw::RawTable<(rustc_middle::ty::context::InternedInSet<rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>>, ())>>::reserve_rehash::<hashbrown::map::make_hasher<rustc_middle::ty::context::InternedInSet<rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>>, rustc_middle::ty::context::InternedInSet<rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>>, (), core::hash::BuildHasherDefault<rustc_hash::FxHasher>>::{closure
    454,831  ???:<rustc_typeck::check::fn_ctxt::FnCtxt>::associated_value
    452,466  ???:<rustc_middle::ty::Predicate as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_middle::ty::fold::BoundVarReplacer>
   -452,466  ???:<rustc_middle::ty::Predicate as rustc_middle::ty::fold::TypeSuperFoldable>::super_fold_with::<rustc_middle::ty::fold::BoundVarReplacer>
   -433,615  ???:<rustc_typeck::check::method::probe::ProbeContext>::assemble_extension_candidates_for_trait
   -407,048  ???:<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_middle::infer::canonical::CanonicalVarInfo>>, <rustc_infer::infer::InferCtxt>::instantiate_canonical_vars<<rustc_infer::infer::InferCtxt>::instantiate_canonical_with_fresh_inference_vars<rustc_middle::ty::context::UserType>::{closure
    400,588  ???:<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_middle::infer::canonical::CanonicalVarInfo>> as core::iter::traits::iterator::Iterator>::fold::<(), core::iter::adapters::map::map_fold<rustc_middle::infer::canonical::CanonicalVarInfo, rustc_middle::ty::subst::GenericArg, (), <rustc_infer::infer::InferCtxt>::instantiate_canonical_vars<<rustc_infer::infer::InferCtxt>::instantiate_canonical_with_fresh_inference_vars<rustc_middle::ty::context::UserType>::{closure
   -389,077  ???:<rustc_infer::infer::InferCtxt>::instantiate_nll_query_response_and_region_obligations::<alloc::vec::Vec<rustc_middle::traits::query::OutlivesBound>>
    379,225  ???:<rustc_middle::ty::fold::RegionFolder as rustc_middle::ty::fold::FallibleTypeFolder>::try_fold_ty
   -379,205  ???:<rustc_middle::ty::Ty as rustc_middle::ty::fold::TypeSuperFoldable>::super_fold_with::<rustc_middle::ty::fold::RegionFolder>
    377,620  ???:<alloc::vec::Vec<rustc_type_ir::UniverseIndex> as alloc::vec::spec_from_iter::SpecFromIter<rustc_type_ir::UniverseIndex, core::iter::adapters::chain::Chain<core::iter::sources::once::Once<rustc_type_ir::UniverseIndex>, core::iter::adapters::map::Map<core::ops::range::Range<u32>, <rustc_infer::infer::InferCtxt>::instantiate_canonical_with_fresh_inference_vars<rustc_middle::ty::context::UserType>::{closure
   -360,979  ???:<rustc_borrowck::borrow_set::BorrowSet>::build
   -349,404  ???:<rustc_typeck::check::method::confirm::ConfirmContext>::confirm
    335,051  ???:<rustc_mir_transform::check_unsafety::UnsafetyChecker as rustc_middle::mir::visit::Visitor>::visit_operand
    334,912  ???:<rustc_trait_selection::traits::select::SelectionContext>::match_impl
    329,027  ???:rustc_query_system::query::plumbing::try_get_cached::<rustc_middle::ty::context::TyCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, rustc_middle::ty::generics::GenericPredicates>, rustc_middle::ty::generics::GenericPredicates, rustc_middle::ty::query::copy<rustc_middle::ty::generics::GenericPredicates>>
    322,563  ???:<rustc_trait_selection::traits::select::SelectionContext>::rematch_impl
    322,165  ???:<rustc_middle::ty::context::CtxtInterners>::intern_predicate
   -322,025  ???:rustc_typeck::check::compare_method::compare_impl_method
   -307,947  ???:rustc_typeck::check::wfcheck::check_where_clauses
    305,696  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::Ty> as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_middle::ty::fold::BoundVarReplacer>
    302,950  ???:<core::result::Result<rustc_middle::ty::Ty, rustc_middle::ty::error::TypeError> as rustc_type_ir::InternIteratorElement<rustc_middle::ty::Ty, rustc_middle::ty::Ty>>::intern_with::<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_middle::ty::Ty>>, core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_middle::ty::Ty>>>, rustc_middle::ty::relate::super_relate_tys<rustc_infer::infer::equate::Equate>::{closure
   -284,280  ???:<rustc_middle::ty::context::TyCtxt>::def_kind::<rustc_span::def_id::LocalDefId>
    277,248  ???:<rustc_borrowck::type_check::TypeChecker>::normalize::<rustc_middle::ty::Ty, rustc_borrowck::type_check::Locations>
   -266,390  ???:rustc_borrowck::type_check::liveness::trace::trace
    246,790  ???:rustc_query_system::query::plumbing::try_get_cached::<rustc_middle::ty::context::TyCtxt, rustc_query_system::query::caches::ArenaCache<rustc_span::def_id::DefId, rustc_middle::ty::generics::Generics>, &rustc_middle::ty::generics::Generics, rustc_middle::ty::query::copy<&rustc_middle::ty::generics::Generics>>
    242,088  ???:<rustc_borrowck::MirBorrowckCtxt>::access_place
    228,735  ???:<rustc_trait_selection::traits::fulfill::FulfillmentContext as rustc_infer::traits::engine::TraitEngine>::register_bound
    226,112  ???:<rustc_index::vec::IndexVec<rustc_middle::ty::sty::BoundVar, rustc_middle::ty::subst::GenericArg> as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_infer::infer::canonical::canonicalizer::Canonicalizer>
    222,460  ???:<alloc::vec::Vec<rustc_middle::ty::subst::GenericArg> as alloc::vec::spec_from_iter::SpecFromIter<rustc_middle::ty::subst::GenericArg, core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_middle::infer::canonical::CanonicalVarInfo>>, <rustc_infer::infer::InferCtxt>::instantiate_canonical_vars<<rustc_infer::infer::InferCtxt>::instantiate_canonical_with_fresh_inference_vars<rustc_middle::ty::context::UserType>::{closure
   -216,640  ???:<rustc_middle::ty::consts::Const as rustc_middle::ty::fold::TypeSuperFoldable>::super_fold_with::<rustc_typeck::check::writeback::EraseEarlyRegions>
