none
--------------------------------------------------------------------------------
Files compared:   results/cgfilt-7425fb293f510a6f138e82a963a3bc599a5b9e1c-cranelift-codegen-0.82.1-Check-IncrFull; results/cgfilt-72423f087b612b625133cd04348ee1bc3d887e43-cranelift-codegen-0.82.1-Check-IncrFull
Command:          /usr/local/rustup/toolchains/7425fb293f510a6f138e82a963a3bc599a5b9e1c/bin/rustc --crate-name cranelift_codegen --edition=2018 src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata -C embed-bitcode=no -C debuginfo=2 -C incremental=/tmp/.tmpE2qgrH/incremental-state --cfg feature="default" --cfg feature="gimli" --cfg feature="std" --cfg feature="unwind" -C metadata=0cdcc3646ceb2365 -C extra-filename=-0cdcc3646ceb2365 --out-dir /tmp/.tmpE2qgrH/target/debug/deps -L dependency=/tmp/.tmpE2qgrH/target/debug/deps --extern cranelift_bforest=/tmp/.tmpE2qgrH/target/debug/deps/libcranelift_bforest-b49c6be2343154ae.rmeta --extern cranelift_codegen_shared=/tmp/.tmpE2qgrH/target/debug/deps/libcranelift_codegen_shared-2571f8f2275f7605.rmeta --extern cranelift_entity=/tmp/.tmpE2qgrH/target/debug/deps/libcranelift_entity-c391de30ba436e90.rmeta --extern gimli=/tmp/.tmpE2qgrH/target/debug/deps/libgimli-1d637e55d6b50218.rmeta --extern log=/tmp/.tmpE2qgrH/target/debug/deps/liblog-e724a7b6173dd03e.rmeta --extern regalloc=/tmp/.tmpE2qgrH/target/debug/deps/libregalloc-60dbf5b5aa455e72.rmeta --extern smallvec=/tmp/.tmpE2qgrH/target/debug/deps/libsmallvec-61e548ea0cf3be6d.rmeta --extern target_lexicon=/tmp/.tmpE2qgrH/target/debug/deps/libtarget_lexicon-57e59898b1f55589.rmeta --cfg feature="x86" -Adeprecated -Aunknown-lints -Zincremental-verify-ich; /usr/local/rustup/toolchains/72423f087b612b625133cd04348ee1bc3d887e43/bin/rustc --crate-name cranelift_codegen --edition=2018 src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata -C embed-bitcode=no -C debuginfo=2 -C incremental=/tmp/.tmpEDp8Z4/incremental-state --cfg feature="default" --cfg feature="gimli" --cfg feature="std" --cfg feature="unwind" -C metadata=0cdcc3646ceb2365 -C extra-filename=-0cdcc3646ceb2365 --out-dir /tmp/.tmpEDp8Z4/target/debug/deps -L dependency=/tmp/.tmpEDp8Z4/target/debug/deps --extern cranelift_bforest=/tmp/.tmpEDp8Z4/target/debug/deps/libcranelift_bforest-b49c6be2343154ae.rmeta --extern cranelift_codegen_shared=/tmp/.tmpEDp8Z4/target/debug/deps/libcranelift_codegen_shared-2571f8f2275f7605.rmeta --extern cranelift_entity=/tmp/.tmpEDp8Z4/target/debug/deps/libcranelift_entity-c391de30ba436e90.rmeta --extern gimli=/tmp/.tmpEDp8Z4/target/debug/deps/libgimli-1d637e55d6b50218.rmeta --extern log=/tmp/.tmpEDp8Z4/target/debug/deps/liblog-e724a7b6173dd03e.rmeta --extern regalloc=/tmp/.tmpEDp8Z4/target/debug/deps/libregalloc-60dbf5b5aa455e72.rmeta --extern smallvec=/tmp/.tmpEDp8Z4/target/debug/deps/libsmallvec-61e548ea0cf3be6d.rmeta --extern target_lexicon=/tmp/.tmpEDp8Z4/target/debug/deps/libtarget_lexicon-57e59898b1f55589.rmeta --cfg feature="x86" -Adeprecated -Aunknown-lints -Zincremental-verify-ich
Data file:        results/cgfilt-diff-7425fb293f510a6f138e82a963a3bc599a5b9e1c-72423f087b612b625133cd04348ee1bc3d887e43-cranelift-codegen-0.82.1-Check-IncrFull
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
217,307,833  PROGRAM TOTALS

--------------------------------------------------------------------------------
Ir           file:function
--------------------------------------------------------------------------------
233,003,852  ???:<rustc_data_structures::obligation_forest::ObligationForest<rustc_trait_selection::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection::traits::fulfill::FulfillProcessor, rustc_data_structures::obligation_forest::Outcome<rustc_trait_selection::traits::fulfill::PendingPredicateObligation, rustc_infer::traits::FulfillmentErrorCode>>
-31,050,386  ???:<rustc_trait_selection::traits::wf::WfPredicates>::nominal_obligations
 28,185,024  ???:<rustc_trait_selection::traits::wf::WfPredicates>::compute
-14,021,415  ???:<rustc_typeck::check::method::probe::ProbeContext>::assemble_inherent_impl_probe
-12,963,890  ???:<alloc::rc::Rc<rustc_middle::traits::ObligationCauseCode> as core::ops::drop::Drop>::drop
 11,709,231  ???:<rustc_typeck::check::method::probe::ProbeContext>::assemble_inherent_impl_candidates_for_type
 -6,427,252  ???:<rustc_mir_transform::function_item_references::FunctionItemReferences as rustc_mir_transform::pass_manager::MirLint>::run_lint
  6,384,272  ???:<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::read_deps::<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::read_index::{closure
  4,920,108  ???:<<rustc_middle::ty::context::TyCtxt>::any_free_region_meets::RegionVisitor<<rustc_middle::ty::context::TyCtxt>::for_each_free_region<rustc_middle::ty::Ty, <rustc_borrowck::type_check::liveness::trace::LivenessContext>::make_all_regions_live<rustc_middle::ty::Ty>::{closure
  4,744,425  ???:core::ptr::drop_in_place::<rustc_middle::traits::ObligationCauseCode>
 -4,724,571  ???:rustc_trait_selection::traits::project::opt_normalize_projection_type
 -4,512,951  ???:<rustc_middle::ty::sty::Region as rustc_middle::ty::fold::TypeFoldable>::visit_with::<<rustc_middle::ty::context::TyCtxt>::any_free_region_meets::RegionVisitor<<rustc_middle::ty::context::TyCtxt>::for_each_free_region<rustc_middle::ty::Ty, <rustc_borrowck::type_check::liveness::trace::LivenessContext>::make_all_regions_live<rustc_middle::ty::Ty>::{closure
 -3,148,057  ???:<dyn rustc_typeck::astconv::AstConv>::res_to_ty
  3,050,848  ???:<rustc_infer::infer::InferCtxt>::commit_if_ok::<(), (), rustc_trait_selection::traits::project::assemble_candidates_from_impls::{closure
  2,658,176  ???:<rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>>::for_item::<<rustc_typeck::check::method::probe::ProbeContext>::fresh_item_substs::{closure
  2,419,630  ???:<alloc::vec::drain_filter::DrainFilter<rustc_infer::traits::Obligation<rustc_middle::ty::Predicate>, rustc_trait_selection::traits::project::opt_normalize_projection_type::{closure
 -1,987,571  ???:rustc_middle::ty::util::fold_list::<rustc_middle::ty::fold::BoundVarReplacer, rustc_middle::ty::Ty, <&rustc_middle::ty::list::List<rustc_middle::ty::Ty> as rustc_middle::ty::fold::TypeFoldable>::try_fold_with<rustc_middle::ty::fold::BoundVarReplacer>::{closure
 -1,914,913  ???:<dyn rustc_typeck::astconv::AstConv>::create_substs_for_generic_args::<<rustc_typeck::check::fn_ctxt::FnCtxt>::instantiate_value_path::CreateCtorSubstsContext>
  1,786,063  ???:<rustc_data_structures::obligation_forest::ObligationForest<rustc_trait_selection::traits::fulfill::PendingPredicateObligation>>::compress::<<rustc_data_structures::obligation_forest::ObligationForest<rustc_trait_selection::traits::fulfill::PendingPredicateObligation>>::process_obligations<rustc_trait_selection::traits::fulfill::FulfillProcessor, rustc_data_structures::obligation_forest::Outcome<rustc_trait_selection::traits::fulfill::PendingPredicateObligation, rustc_infer::traits::FulfillmentErrorCode>>::{closure
  1,548,848  ???:<rustc_data_structures::intern::Interned<rustc_data_structures::intern::WithStableHash<rustc_middle::ty::TyS>> as rustc_data_structures::stable_hasher::HashStable<rustc_query_system::ich::hcx::StableHashingContext>>::hash_stable
  1,490,100  ???:<rustc_middle::hir::map::Map>::local_def_id_to_hir_id
  1,452,866  ???:<rustc_borrowck::type_check::TypeChecker>::typeck_mir
  1,362,860  ???:rustc_typeck::collect::gather_explicit_predicates_of
 -1,328,076  ???:<rustc_middle::mir::BasicBlockData>::expand_statements::<<rustc_mir_transform::deaggregator::Deaggregator as rustc_middle::mir::MirPass>::run_pass::{closure
  1,322,851  ???:<rustc_mir_transform::deaggregator::Deaggregator as rustc_middle::mir::MirPass>::run_pass
 -1,189,433  ???:<rustc_borrowck::MirBorrowckCtxt as rustc_mir_dataflow::framework::visitor::ResultsVisitor>::visit_statement_before_primary_effect
 -1,046,494  ???:rustc_mir_transform::check_unsafety::unsafety_check_result
  1,033,716  ???:rustc_typeck::collect::generics_of
 -1,028,421  ???:<rustc_middle::ty::Ty as rustc_middle::ty::fold::TypeSuperFoldable>::super_visit_with::<<rustc_middle::ty::context::TyCtxt>::any_free_region_meets::RegionVisitor<<rustc_middle::ty::context::TyCtxt>::for_each_free_region<rustc_middle::ty::Ty, <rustc_borrowck::type_check::liveness::trace::LivenessContext>::make_all_regions_live<rustc_middle::ty::Ty>::{closure
   -999,091  ???:<rustc_typeck::check::method::probe::ProbeContext>::assemble_inherent_candidates
   -990,917  ???:_rjem_je_arena_cache_bin_fill_small
    948,985  ???:<(rustc_middle::ty::sty::FnSig, rustc_middle::ty::InstantiatedPredicates) as rustc_middle::ty::fold::TypeFoldable>::fold_with::<rustc_infer::infer::resolve::OpportunisticVarResolver>
   -948,862  ???:<rustc_infer::infer::InferCtxt>::instantiate_nll_query_response_and_region_obligations::<()>
   -900,562  ???:<rustc_middle::ty::context::TyCtxt>::def_kind::<rustc_span::def_id::LocalDefId>
    891,144  ???:<rustc_typeck::check::method::probe::ProbeContext>::assemble_inherent_candidates_for_incoherent_ty
   -850,538  ???:<rustc_typeck::check::method::probe::ProbeContext>::xform_self_ty
   -846,347  ???:<dyn rustc_typeck::astconv::AstConv>::create_substs_for_generic_args::<<rustc_typeck::check::method::confirm::ConfirmContext>::instantiate_method_substs::MethodSubstsCtxt>
   -837,876  ???:<rustc_typeck::check::fn_ctxt::FnCtxt>::instantiate_value_path
    821,284  ???:<rustc_typeck::check::fn_ctxt::FnCtxt>::associated_value
    778,491  ???:<rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>>::for_item::<rustc_typeck::check::wfcheck::check_where_clauses::{closure
   -765,861  ???:<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_query_system::dep_graph::graph::DepNodeIndex>> as core::iter::traits::iterator::Iterator>::fold::<(), core::iter::adapters::map::map_fold<rustc_query_system::dep_graph::graph::DepNodeIndex, (rustc_query_system::dep_graph::graph::DepNodeIndex, ()), (), <hashbrown::set::HashSet<rustc_query_system::dep_graph::graph::DepNodeIndex, core::hash::BuildHasherDefault<rustc_hash::FxHasher>> as core::iter::traits::collect::Extend<rustc_query_system::dep_graph::graph::DepNodeIndex>>::extend<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_query_system::dep_graph::graph::DepNodeIndex>>>::{closure
    723,788  ???:<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_query_system::dep_graph::graph::DepNodeIndex>>, <hashbrown::set::HashSet<rustc_query_system::dep_graph::graph::DepNodeIndex, core::hash::BuildHasherDefault<rustc_hash::FxHasher>> as core::iter::traits::collect::Extend<rustc_query_system::dep_graph::graph::DepNodeIndex>>::extend<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_query_system::dep_graph::graph::DepNodeIndex>>>::{closure
    714,324  ???:<rustc_trait_selection::traits::select::SelectionContext>::select
   -694,977  ???:<[rustc_middle::ty::subst::GenericArg] as rustc_data_structures::stable_hasher::HashStable<rustc_query_system::ich::hcx::StableHashingContext>>::hash_stable
   -669,169  ???:<rustc_typeck::check::writeback::WritebackCx as rustc_hir::intravisit::Visitor>::visit_expr
   -649,328  ???:rustc_typeck::outlives::implicit_infer::infer_predicates
    618,295  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::Ty> as rustc_data_structures::stable_hasher::HashStable<rustc_query_system::ich::hcx::StableHashingContext>>::hash_stable
   -586,837  ???:rustc_trait_selection::traits::project::normalize::<(rustc_middle::ty::sty::FnSig, rustc_middle::ty::InstantiatedPredicates)>
    582,085  ???:core::ptr::drop_in_place::<rustc_infer::infer::InferCtxtBuilder>
   -559,957  ???:core::ptr::drop_in_place::<rustc_typeck::check::inherited::InheritedBuilder>
   -512,878  ???:_rjem_je_malloc_default
    500,205  ???:<rustc_infer::infer::InferCtxt as rustc_trait_selection::infer::InferCtxtExt>::partially_normalize_associated_types_in::<(rustc_middle::ty::sty::FnSig, rustc_middle::ty::InstantiatedPredicates)>
   -497,598  ???:<std::thread::local::LocalKey<core::cell::RefCell<std::collections::hash::map::HashMap<(usize, usize, rustc_data_structures::stable_hasher::HashingControls), rustc_data_structures::fingerprint::Fingerprint, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>>>::with::<<&rustc_middle::ty::list::List<rustc_middle::ty::Ty> as rustc_data_structures::stable_hasher::HashStable<rustc_query_system::ich::hcx::StableHashingContext>>::hash_stable::{closure
    485,402  ???:rustc_query_system::query::plumbing::try_get_cached::<rustc_middle::ty::context::TyCtxt, rustc_query_system::query::caches::ArenaCache<rustc_span::def_id::DefId, rustc_middle::ty::generics::Generics>, &rustc_middle::ty::generics::Generics, rustc_middle::ty::query::copy<&rustc_middle::ty::generics::Generics>>
   -458,788  ???:<hashbrown::raw::RawTable<(rustc_middle::ty::context::InternedInSet<rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>>, ())>>::reserve_rehash::<hashbrown::map::make_hasher<rustc_middle::ty::context::InternedInSet<rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>>, rustc_middle::ty::context::InternedInSet<rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>>, (), core::hash::BuildHasherDefault<rustc_hash::FxHasher>>::{closure
   -452,466  ???:<rustc_middle::ty::Predicate as rustc_middle::ty::fold::TypeSuperFoldable>::super_fold_with::<rustc_middle::ty::fold::BoundVarReplacer>
    452,466  ???:<rustc_middle::ty::Predicate as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_middle::ty::fold::BoundVarReplacer>
   -451,147  ???:<rustc_typeck::check::method::probe::ProbeContext>::assemble_extension_candidates_for_trait
   -407,048  ???:<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_middle::infer::canonical::CanonicalVarInfo>>, <rustc_infer::infer::InferCtxt>::instantiate_canonical_vars<<rustc_infer::infer::InferCtxt>::instantiate_canonical_with_fresh_inference_vars<rustc_middle::ty::context::UserType>::{closure
    400,588  ???:<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_middle::infer::canonical::CanonicalVarInfo>> as core::iter::traits::iterator::Iterator>::fold::<(), core::iter::adapters::map::map_fold<rustc_middle::infer::canonical::CanonicalVarInfo, rustc_middle::ty::subst::GenericArg, (), <rustc_infer::infer::InferCtxt>::instantiate_canonical_vars<<rustc_infer::infer::InferCtxt>::instantiate_canonical_with_fresh_inference_vars<rustc_middle::ty::context::UserType>::{closure
   -389,077  ???:<rustc_infer::infer::InferCtxt>::instantiate_nll_query_response_and_region_obligations::<alloc::vec::Vec<rustc_middle::traits::query::OutlivesBound>>
    379,225  ???:<rustc_middle::ty::fold::RegionFolder as rustc_middle::ty::fold::FallibleTypeFolder>::try_fold_ty
   -379,205  ???:<rustc_middle::ty::Ty as rustc_middle::ty::fold::TypeSuperFoldable>::super_fold_with::<rustc_middle::ty::fold::RegionFolder>
    377,620  ???:<alloc::vec::Vec<rustc_type_ir::UniverseIndex> as alloc::vec::spec_from_iter::SpecFromIter<rustc_type_ir::UniverseIndex, core::iter::adapters::chain::Chain<core::iter::sources::once::Once<rustc_type_ir::UniverseIndex>, core::iter::adapters::map::Map<core::ops::range::Range<u32>, <rustc_infer::infer::InferCtxt>::instantiate_canonical_with_fresh_inference_vars<rustc_middle::ty::context::UserType>::{closure
   -360,979  ???:<rustc_borrowck::borrow_set::BorrowSet>::build
   -357,151  ???:<[rustc_middle::ty::Ty] as rustc_serialize::serialize::Encodable<rustc_query_impl::on_disk_cache::CacheEncoder>>::encode
   -349,877  ???:rustc_typeck::check::compare_method::compare_impl_method
    349,819  ???:rustc_query_system::query::plumbing::try_get_cached::<rustc_middle::ty::context::TyCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, rustc_middle::ty::generics::GenericPredicates>, rustc_middle::ty::generics::GenericPredicates, rustc_middle::ty::query::copy<rustc_middle::ty::generics::GenericPredicates>>
    349,788  ???:<rustc_middle::ty::Ty as rustc_serialize::serialize::Encodable<rustc_query_impl::on_disk_cache::CacheEncoder>>::encode
   -349,404  ???:<rustc_typeck::check::method::confirm::ConfirmContext>::confirm
    335,011  ???:<rustc_mir_transform::check_unsafety::UnsafetyChecker as rustc_middle::mir::visit::Visitor>::visit_operand
    334,912  ???:<rustc_trait_selection::traits::select::SelectionContext>::match_impl
   -328,951  ???:rustc_typeck::check::wfcheck::check_where_clauses
    322,563  ???:<rustc_trait_selection::traits::select::SelectionContext>::rematch_impl
    304,277  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::Ty> as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_middle::ty::fold::BoundVarReplacer>
    302,950  ???:<core::result::Result<rustc_middle::ty::Ty, rustc_middle::ty::error::TypeError> as rustc_type_ir::InternIteratorElement<rustc_middle::ty::Ty, rustc_middle::ty::Ty>>::intern_with::<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_middle::ty::Ty>>, core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_middle::ty::Ty>>>, rustc_middle::ty::relate::super_relate_tys<rustc_infer::infer::equate::Equate>::{closure
   -295,344  ???:<rustc_middle::ty::subst::GenericArg as rustc_data_structures::stable_hasher::HashStable<rustc_query_system::ich::hcx::StableHashingContext>>::hash_stable
   -284,334  ???:<[rustc_middle::ty::Ty] as rustc_data_structures::stable_hasher::HashStable<rustc_query_system::ich::hcx::StableHashingContext>>::hash_stable
    277,248  ???:<rustc_borrowck::type_check::TypeChecker>::normalize::<rustc_middle::ty::Ty, rustc_borrowck::type_check::Locations>
    272,782  ???:<rustc_middle::ty::context::TyCtxt>::bound_impl_trait_ref
   -266,390  ???:rustc_borrowck::type_check::liveness::trace::trace
    242,088  ???:<rustc_borrowck::MirBorrowckCtxt>::access_place
   -228,436  ???:<dyn rustc_typeck::astconv::AstConv>::instantiate_poly_trait_ref_inner
    226,112  ???:<rustc_index::vec::IndexVec<rustc_middle::ty::sty::BoundVar, rustc_middle::ty::subst::GenericArg> as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_infer::infer::canonical::canonicalizer::Canonicalizer>
    225,968  ???:rustc_query_system::query::plumbing::try_get_cached::<rustc_middle::ty::context::TyCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::FnSig>>, rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::FnSig>, rustc_middle::ty::query::copy<rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::FnSig>>>
    222,460  ???:<alloc::vec::Vec<rustc_middle::ty::subst::GenericArg> as alloc::vec::spec_from_iter::SpecFromIter<rustc_middle::ty::subst::GenericArg, core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_middle::infer::canonical::CanonicalVarInfo>>, <rustc_infer::infer::InferCtxt>::instantiate_canonical_vars<<rustc_infer::infer::InferCtxt>::instantiate_canonical_with_fresh_inference_vars<rustc_middle::ty::context::UserType>::{closure
    218,050  ???:<rustc_trait_selection::traits::fulfill::FulfillmentContext as rustc_infer::traits::engine::TraitEngine>::register_bound
