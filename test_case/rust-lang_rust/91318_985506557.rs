
--------------------------------------------------------------------------------
Files compared:   results/cgfilt-e5038e20999eef35260b070189883edc2a8a34b2-deeply-nested-Check-Full; results/cgfilt-acbe4443cc4c9695c0b74a7b64b60333c990a400-deeply-nested-Check-Full
Command:          /usr/local/rustup/toolchains/e5038e20999eef35260b070189883edc2a8a34b2/bin/rustc --crate-name deeply_nested src/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata -C embed-bitcode=no -C debuginfo=2 -C metadata=3e5be4b5836f970a -C extra-filename=-3e5be4b5836f970a --out-dir /tmp/.tmpB5v6UE/target/debug/deps -L dependency=/tmp/.tmpB5v6UE/target/debug/deps -Adeprecated -Aunknown-lints; /usr/local/rustup/toolchains/acbe4443cc4c9695c0b74a7b64b60333c990a400/bin/rustc --crate-name deeply_nested src/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata -C embed-bitcode=no -C debuginfo=2 -C metadata=3e5be4b5836f970a -C extra-filename=-3e5be4b5836f970a --out-dir /tmp/.tmpL3xGNi/target/debug/deps -L dependency=/tmp/.tmpL3xGNi/target/debug/deps -Adeprecated -Aunknown-lints
Data file:        results/cgdiff-e5038e20999eef35260b070189883edc2a8a34b2-acbe4443cc4c9695c0b74a7b64b60333c990a400-deeply-nested-Check-Full
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
506,098 (100.0%)  PROGRAM TOTALS

--------------------------------------------------------------------------------
Ir                   file:function
--------------------------------------------------------------------------------
-1,774,783 (-350.7%)  ???:<&rustc_middle::ty::TyS as rustc_middle::ty::fold::TypeFoldable>::fold_with::<rustc_infer::infer::freshen::TypeFreshener>
 1,773,875 (350.5%)  ???:<&rustc_middle::ty::TyS as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_infer::infer::freshen::TypeFreshener>
   889,680 (175.8%)  ???:<&rustc_middle::ty::TyS as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_infer::infer::resolve::OpportunisticVarResolver>
 -520,455 (-102.8%)  ???:<&rustc_middle::ty::TyS as rustc_middle::ty::fold::TypeFoldable>::super_fold_with::<rustc_infer::infer::resolve::OpportunisticVarResolver>
  -218,229 (-43.1%)  ???:<rustc_middle::ty::subst::GenericArg as rustc_middle::ty::fold::TypeFoldable>::fold_with::<rustc_middle::ty::subst::SubstFolder>
   184,909 (36.54%)  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_middle::ty::subst::SubstFolder>
  -152,920 (-30.2%)  ???:<core::result::Result<rustc_middle::ty::subst::GenericArg, rustc_middle::ty::error::TypeError> as rustc_middle::ty::context::InternIteratorElement<rustc_middle::ty::subst::GenericArg, &rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>>>::intern_with::<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::zip::Zip<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_middle::ty::subst::GenericArg>>, core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_middle::ty::subst::GenericArg>>>>, rustc_middle::ty::relate::relate_substs<rustc_infer::infer::equate::Equate>::{closure
   150,873 (29.81%)  ???:<rustc_infer::infer::combine::Generalizer as rustc_middle::ty::relate::TypeRelation>::relate_item_substs
  -137,217 (-27.1%)  ???:<&rustc_middle::ty::TyS as rustc_middle::ty::fold::TypeFoldable>::super_fold_with::<rustc_infer::infer::resolve::FullTypeResolver>
   134,006 (26.48%)  ???:<alloc::rc::Rc<rustc_middle::traits::ObligationCauseData> as core::ops::drop::Drop>::drop
   133,760 (26.43%)  ???:<rustc_typeck::check::fn_ctxt::FnCtxt>::normalize_associated_types_in::<(rustc_middle::ty::sty::FnSig, rustc_middle::ty::InstantiatedPredicates)>
  -131,312 (-25.9%)  ???:<rustc_typeck::check::inherited::Inherited>::normalize_associated_types_in::<(rustc_middle::ty::sty::FnSig, rustc_middle::ty::InstantiatedPredicates)>
  -127,131 (-25.1%)  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_middle::ty::fold::TypeFoldable>::fold_with::<rustc_middle::ty::fold::BoundVarReplacer>
   112,957 (22.32%)  ???:<&rustc_middle::ty::TyS as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_middle::ty::fold::RegionFolder>
  -112,073 (-22.1%)  ???:<&rustc_middle::ty::TyS as rustc_middle::ty::fold::TypeFoldable>::super_fold_with::<rustc_middle::ty::fold::RegionFolder>
   111,299 (21.99%)  ???:<&rustc_middle::ty::TyS as rustc_middle::ty::fold::TypeFoldable>::super_fold_with::<rustc_middle::ty::fold::BoundVarReplacer>
   109,329 (21.60%)  ???:rustc_middle::ty::relate::relate_substs::<rustc_infer::infer::equate::Equate>
    99,946 (19.75%)  ???:<&rustc_middle::ty::TyS as rustc_middle::ty::fold::TypeFoldable>::try_super_fold_with::<rustc_infer::infer::resolve::FullTypeResolver>
    91,584 (18.10%)  ???:<rustc_middle::ty::subst::GenericArg as rustc_middle::ty::relate::Relate>::relate::<rustc_infer::infer::equate::Equate>
    72,838 (14.39%)  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_infer::infer::resolve::OpportunisticVarResolver>
    65,131 (12.87%)  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_infer::infer::resolve::FullTypeResolver>
   -62,867 (-12.4%)  ???:rustc_middle::ty::relate::relate_substs::<rustc_infer::infer::combine::Generalizer>
    55,678 (11.00%)  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_infer::infer::canonical::canonicalizer::Canonicalizer>
   -40,290 (-7.96%)  ???:<rustc_middle::ty::subst::SubstFolder as rustc_middle::ty::fold::TypeFolder>::fold_ty
   -39,058 (-7.72%)  ???:<rustc_middle::ty::context::TyCtxt>::intern_substs
   -36,716 (-7.25%)  ???:<rustc_infer::traits::project::ProjectionCache>::try_start
   -34,104 (-6.74%)  ???:rustc_middle::ty::relate::super_relate_tys::<rustc_infer::infer::equate::Equate>
   -31,992 (-6.32%)  ???:<&rustc_middle::ty::TyS as rustc_middle::ty::fold::TypeFoldable>::super_fold_with::<rustc_infer::infer::canonical::canonicalizer::Canonicalizer>
   -29,205 (-5.77%)  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_middle::ty::fold::TypeFoldable>::fold_with::<rustc_middle::ty::subst::SubstFolder>
   -24,130 (-4.77%)  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_middle::ty::fold::TypeFoldable>::fold_with::<rustc_trait_selection::traits::project::AssocTypeNormalizer>
    24,130 ( 4.77%)  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_trait_selection::traits::project::AssocTypeNormalizer>
    23,297 ( 4.60%)  ???:<rustc_infer::infer::InferCtxt>::region_constraints_added_in_snapshot
   -18,277 (-3.61%)  ???:<&rustc_middle::ty::TyS as rustc_middle::ty::fold::TypeFoldable>::super_fold_with::<rustc_infer::infer::freshen::TypeFreshener>
   -18,260 (-3.61%)  ???:<std::collections::hash::map::HashMap<rustc_infer::traits::project::ProjectionCacheKey, rustc_infer::traits::project::ProjectionCacheEntry, core::hash::BuildHasherDefault<rustc_hash::FxHasher>> as ena::undo_log::Rollback<rustc_data_structures::snapshot_map::UndoLog<rustc_infer::traits::project::ProjectionCacheKey, rustc_infer::traits::project::ProjectionCacheEntry>>>::reverse
    18,111 ( 3.58%)  ???:<hashbrown::map::RawEntryBuilderMut<rustc_middle::ty::context::Interned<rustc_middle::ty::TyS>, (), core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::from_hash::<hashbrown::map::equivalent<rustc_middle::ty::sty::TyKind, rustc_middle::ty::context::Interned<rustc_middle::ty::TyS>>::{closure
   -17,620 (-3.48%)  ???:<&rustc_middle::ty::TyS as rustc_middle::ty::fold::TypeFoldable>::super_fold_with::<rustc_middle::ty::subst::SubstFolder>
    15,290 ( 3.02%)  ???:<rustc_middle::ty::fold::Shifter as rustc_middle::ty::fold::FallibleTypeFolder>::try_fold_ty
   -14,595 (-2.88%)  ???:<&rustc_middle::ty::TyS as rustc_middle::ty::fold::TypeFoldable>::super_fold_with::<rustc_middle::ty::fold::Shifter>
   -14,462 (-2.86%)  ???:<rustc_mir_build::thir::cx::Cx>::mirror_expr_inner
   -13,933 (-2.75%)  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_middle::ty::fold::TypeFoldable>::fold_with::<rustc_infer::infer::resolve::FullTypeResolver>
   -13,796 (-2.73%)  ???:<rustc_middle::ty::Predicate as rustc_middle::ty::fold::TypeFoldable>::fold_with::<rustc_infer::infer::resolve::OpportunisticVarResolver>
    13,796 ( 2.73%)  ???:<rustc_middle::ty::Predicate as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_infer::infer::resolve::OpportunisticVarResolver>
    12,652 ( 2.50%)  ???:<rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::FallibleTypeFolder>::try_fold_ty
   -12,585 (-2.49%)  ???:<rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_ty
   -12,516 (-2.47%)  ???:<rustc_middle::ty::sty::TraitRef as rustc_middle::ty::fold::TypeFoldable>::fold_with::<rustc_middle::ty::subst::SubstFolder>
    11,334 ( 2.24%)  ???:<rustc_trait_selection::traits::fulfill::FulfillmentContext as rustc_infer::traits::engine::TraitEngine>::register_predicate_obligation
   -11,112 (-2.20%)  ???:<rustc_middle::ty::InstantiatedPredicates as rustc_middle::ty::fold::TypeFoldable>::fold_with::<rustc_infer::infer::resolve::OpportunisticVarResolver>
    10,949 ( 2.16%)  ???:<rustc_mir_build::thir::cx::Cx>::make_mirror_unadjusted
    10,552 ( 2.08%)  ???:<alloc::vec::Vec<rustc_middle::ty::Predicate> as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_infer::infer::resolve::OpportunisticVarResolver>
    -9,148 (-1.81%)  ???:<alloc::vec::Vec<rustc_middle::ty::Predicate> as alloc::vec::spec_extend::SpecExtend<rustc_middle::ty::Predicate, core::iter::adapters::map::Map<core::slice::iter::Iter<(rustc_middle::ty::Predicate, rustc_span::span_encoding::Span)>, <rustc_middle::ty::generics::GenericPredicates>::instantiate_into::{closure
    -9,087 (-1.80%)  ???:<rustc_infer::infer::freshen::TypeFreshener as rustc_middle::ty::fold::TypeFolder>::fold_ty
     8,539 ( 1.69%)  ???:rustc_data_structures::stack::ensure_sufficient_stack::<rustc_infer::traits::project::Normalized<rustc_middle::ty::sty::TraitRef>, <rustc_trait_selection::traits::select::SelectionContext>::match_impl::{closure
    -8,358 (-1.65%)  ???:llvm::PassRegistry::registerPass(llvm::PassInfo const&, bool)
     8,278 ( 1.64%)  ???:<rustc_middle::ty::sty::TraitRef as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_middle::ty::subst::SubstFolder>
     7,634 ( 1.51%)  ???:<rustc_middle::ty::inhabitedness::def_id_forest::DefIdForest>::union::<core::iter::adapters::map::Map<core::slice::iter::Iter<rustc_middle::ty::FieldDef>, <rustc_middle::ty::VariantDef>::uninhabited_from::{closure
    -6,616 (-1.31%)  ???:<rustc_infer::infer::equate::Equate as rustc_middle::ty::relate::TypeRelation>::tys
     6,184 ( 1.22%)  ???:<&rustc_middle::ty::TyS as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_infer::infer::canonical::canonicalizer::Canonicalizer>
    -5,377 (-1.06%)  ???:<&mut <rustc_middle::ty::adt::AdtDef>::uninhabited_from::{closure
     5,173 ( 1.02%)  ???:<rustc_trait_selection::traits::fulfill::FulfillProcessor>::progress_changed_obligations
     5,167 ( 1.02%)  ???:<alloc::vec::Vec<rustc_infer::traits::Obligation<rustc_middle::ty::Predicate>>>::retain::<<rustc_trait_selection::traits::select::SelectionContext>::impl_or_trait_obligations::{closure
     5,136 ( 1.01%)  ???:rustc_trait_selection::traits::project::normalize::<(rustc_middle::ty::sty::FnSig, rustc_middle::ty::InstantiatedPredicates)>
    -4,773 (-0.94%)  ???:<&mut <rustc_middle::ty::VariantDef>::uninhabited_from::{closure
     4,606 ( 0.91%)  ???:<rustc_middle::ty::subst::SubstFolder as rustc_middle::ty::fold::FallibleTypeFolder>::try_fold_ty
    -4,569 (-0.90%)  ???:<rustc_data_structures::obligation_forest::ObligationForest<rustc_trait_selection::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection::traits::fulfill::FulfillProcessor, rustc_data_structures::obligation_forest::Outcome<rustc_trait_selection::traits::fulfill::PendingPredicateObligation, rustc_infer::traits::FulfillmentErrorCode>>
    -4,513 (-0.89%)  ???:core::ptr::drop_in_place::<rustc_middle::traits::ObligationCauseCode>
    -4,485 (-0.89%)  ???:<rustc_infer::infer::sub::Sub as rustc_middle::ty::relate::TypeRelation>::relate::<&rustc_middle::ty::TyS>
     4,219 ( 0.83%)  ???:<rustc_mir_build::thir::cx::Cx>::mirror_expr
    -4,208 (-0.83%)  ???:<rustc_middle::ty::fold::BoundVarReplacer as rustc_middle::ty::fold::TypeFolder>::fold_predicate
     4,208 ( 0.83%)  ???:<rustc_middle::ty::fold::BoundVarReplacer as rustc_middle::ty::fold::FallibleTypeFolder>::try_fold_predicate
     3,770 ( 0.74%)  ???:rustc_middle::ty::util::fold_list::<rustc_infer::infer::freshen::TypeFreshener, rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>, <&rustc_middle::ty::list::List<rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>> as rustc_middle::ty::fold::TypeFoldable>::try_super_fold_with<rustc_infer::infer::freshen::TypeFreshener>::{closure
    -3,770 (-0.74%)  ???:rustc_middle::ty::util::fold_list::<rustc_infer::infer::freshen::TypeFreshener, rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>, <&rustc_middle::ty::list::List<rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>> as rustc_middle::ty::fold::TypeFoldable>::super_fold_with<rustc_infer::infer::freshen::TypeFreshener>::{closure
     3,726 ( 0.74%)  ???:rustc_middle::ty::util::fold_list::<rustc_infer::infer::resolve::OpportunisticVarResolver, rustc_middle::ty::Predicate, <&rustc_middle::ty::list::List<rustc_middle::ty::Predicate> as rustc_middle::ty::fold::TypeFoldable>::try_super_fold_with<rustc_infer::infer::resolve::OpportunisticVarResolver>::{closure
    -3,726 (-0.74%)  ???:rustc_middle::ty::util::fold_list::<rustc_infer::infer::resolve::OpportunisticVarResolver, rustc_middle::ty::Predicate, <&rustc_middle::ty::list::List<rustc_middle::ty::Predicate> as rustc_middle::ty::fold::TypeFoldable>::super_fold_with<rustc_infer::infer::resolve::OpportunisticVarResolver>::{closure
    -3,584 (-0.71%)  ???:<rustc_trait_selection::traits::project::AssocTypeNormalizer>::fold::<(rustc_middle::ty::sty::FnSig, rustc_middle::ty::InstantiatedPredicates)>
     3,184 ( 0.63%)  ???:<rustc_data_structures::snapshot_map::SnapshotMap<rustc_infer::traits::project::ProjectionCacheKey, rustc_infer::traits::project::ProjectionCacheEntry, &mut std::collections::hash::map::HashMap<rustc_infer::traits::project::ProjectionCacheKey, rustc_infer::traits::project::ProjectionCacheEntry, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>, &mut rustc_infer::infer::undo_log::InferCtxtUndoLogs>>::insert
     3,008 ( 0.59%)  ???:<rustc_infer::infer::resolve::OpportunisticVarResolver as rustc_middle::ty::fold::FallibleTypeFolder>::try_fold_binder::<rustc_middle::ty::sty::FnSig>
     2,938 ( 0.58%)  ???:<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>>, alloc::vec::into_iter::IntoIter<rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>>>, <&rustc_middle::ty::list::List<rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>> as rustc_middle::ty::relate::Relate>::relate<rustc_infer::infer::combine::Generalizer>::{closure
     2,890 ( 0.57%)  ???:<&rustc_middle::ty::consts::Const as rustc_middle::ty::fold::TypeFoldable>::fold_with::<rustc_middle::ty::fold::RegionFolder>
    -2,800 (-0.55%)  ./elf/dl-lookup.c:_dl_lookup_symbol_x
    -2,730 (-0.54%)  ???:<core::result::Result<rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>, rustc_middle::ty::error::TypeError> as rustc_middle::ty::context::InternIteratorElement<rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>, &rustc_middle::ty::list::List<rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>>>>::intern_with::<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>>, alloc::vec::into_iter::IntoIter<rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>>>, <&rustc_middle::ty::list::List<rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>> as rustc_middle::ty::relate::Relate>::relate<rustc_infer::infer::combine::Generalizer>::{closure
    -2,712 (-0.54%)  ???:<rustc_infer::infer::resolve::FullTypeResolver as rustc_middle::ty::fold::TypeFolder>::fold_ty
     2,712 ( 0.54%)  ???:<rustc_infer::infer::resolve::FullTypeResolver as rustc_middle::ty::fold::FallibleTypeFolder>::try_fold_ty
     2,694 ( 0.53%)  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_middle::ty::fold::BoundVarReplacer>
    -2,516 (-0.50%)  ???:<rustc_middle::ty::context::TyCtxt>::fold_regions::<&rustc_middle::ty::consts::Const, rustc_borrowck::renumber::renumber_regions<&rustc_middle::ty::consts::Const>::{closure
     2,499 ( 0.49%)  ???:<alloc::collections::btree::map::entry::Entry<rustc_middle::ty::sty::BoundRegion, &rustc_middle::ty::sty::RegionKind>>::or_insert_with::<<rustc_middle::ty::context::TyCtxt>::replace_late_bound_regions<rustc_middle::ty::PredicateKind, <rustc_middle::ty::context::TyCtxt>::anonymize_late_bound_regions<rustc_middle::ty::PredicateKind>::{closure
    -2,490 (-0.49%)  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_middle::ty::fold::TypeFoldable>::fold_with::<rustc_infer::infer::canonical::canonicalizer::Canonicalizer>
     2,449 ( 0.48%)  ???:<rustc_index::vec::IndexVec<rustc_middle::ty::sty::BoundVar, rustc_middle::ty::subst::GenericArg> as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_infer::infer::canonical::canonicalizer::Canonicalizer>
    -2,449 (-0.48%)  ???:<rustc_index::vec::IndexVec<rustc_middle::ty::sty::BoundVar, rustc_middle::ty::subst::GenericArg> as rustc_middle::ty::fold::TypeFoldable>::fold_with::<rustc_infer::infer::canonical::canonicalizer::Canonicalizer>
    -2,337 (-0.46%)  ???:<rustc_middle::ty::fold::BoundVarReplacer as rustc_middle::ty::fold::TypeFolder>::fold_ty
    -2,304 (-0.46%)  ???:<&rustc_middle::ty::list::List<&rustc_middle::ty::TyS> as rustc_middle::ty::fold::TypeFoldable>::fold_with::<rustc_infer::infer::resolve::OpportunisticVarResolver>
     2,304 ( 0.46%)  ???:<&rustc_middle::ty::list::List<&rustc_middle::ty::TyS> as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_infer::infer::resolve::OpportunisticVarResolver>
     2,296 ( 0.45%)  ???:rustc_infer::traits::util::elaborate_obligations
    -2,243 (-0.44%)  ./string/../sysdeps/x86_64/multiarch/memmove-vec-unaligned-erms.S:__memcpy_sse2_unaligned_erms
    -2,240 (-0.44%)  ???:<rustc_infer::traits::project::ProjectionCache>::ambiguous
    -2,224 (-0.44%)  ???:<rustc_middle::ty::Predicate as core::cmp::PartialEq>::eq
    -2,200 (-0.43%)  ???:<object::elf::FileHeader64<object::endian::Endianness> as object::read::elf::file::FileHeader>::sections::<&[u8]>
     2,179 ( 0.43%)  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_middle::ty::fold::TypeFoldable>::try_super_fold_with::<rustc_trait_selection::traits::query::normalize::QueryNormalizer>
    -2,179 (-0.43%)  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_middle::ty::fold::TypeFoldable>::super_fold_with::<rustc_trait_selection::traits::query::normalize::QueryNormalizer>
     2,115 ( 0.42%)  ???:<alloc::collections::btree::node::Handle<alloc::collections::btree::node::NodeRef<alloc::collections::btree::node::marker::Mut, rustc_middle::ty::sty::BoundRegion, &rustc_middle::ty::sty::RegionKind, alloc::collections::btree::node::marker::Leaf>, alloc::collections::btree::node::marker::Edge>>::insert_recursing
     2,080 ( 0.41%)  ???:<rustc_infer::infer::InferCtxt>::super_combine_tys::<rustc_infer::infer::sub::Sub>
     2,080 ( 0.41%)  ???:<&rustc_middle::ty::list::List<&rustc_middle::ty::TyS> as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_trait_selection::traits::project::AssocTypeNormalizer>
    -2,080 (-0.41%)  ???:<&rustc_middle::ty::list::List<&rustc_middle::ty::TyS> as rustc_middle::ty::fold::TypeFoldable>::fold_with::<rustc_trait_selection::traits::project::AssocTypeNormalizer>
     2,006 ( 0.40%)  ???:<rustc_trait_selection::traits::fulfill::FulfillmentContext as rustc_infer::traits::engine::TraitEngine>::select_where_possible
    -1,964 (-0.39%)  ???:<rustc_infer::traits::project::ProjectionCache>::insert_ty
     1,934 ( 0.38%)  ???:<rustc_middle::ty::inhabitedness::def_id_forest::DefIdForest>::intersection::<core::iter::adapters::map::Map<core::slice::iter::Iter<rustc_middle::ty::VariantDef>, <rustc_middle::ty::adt::AdtDef>::uninhabited_from::{closure
    -1,932 (-0.38%)  ???:llvm::DenseMap<void const*, llvm::PassInfo const*, llvm::DenseMapInfo<void const*>, llvm::detail::DenseMapPair<void const*, llvm::PassInfo const*> >::grow(unsigned int)
    -1,880 (-0.37%)  ???:<alloc::collections::btree::map::entry::VacantEntry<rustc_middle::ty::sty::BoundRegion, &rustc_middle::ty::sty::RegionKind>>::insert
     1,738 ( 0.34%)  ???:rustc_trait_selection::traits::specialize::translate_substs
     1,712 ( 0.34%)  ???:rustc_middle::ty::util::fold_list::<rustc_trait_selection::traits::query::normalize::QueryNormalizer, &rustc_middle::ty::TyS, <&rustc_middle::ty::list::List<&rustc_middle::ty::TyS> as rustc_middle::ty::fold::TypeFoldable>::try_super_fold_with<rustc_trait_selection::traits::query::normalize::QueryNormalizer>::{closure
    -1,712 (-0.34%)  ???:rustc_middle::ty::util::fold_list::<rustc_trait_selection::traits::query::normalize::QueryNormalizer, &rustc_middle::ty::TyS, <&rustc_middle::ty::list::List<&rustc_middle::ty::TyS> as rustc_middle::ty::fold::TypeFoldable>::super_fold_with<rustc_trait_selection::traits::query::normalize::QueryNormalizer>::{closure
    -1,656 (-0.33%)  ???:rustc_middle::ty::util::fold_list::<rustc_infer::infer::canonical::canonicalizer::Canonicalizer, rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>, <&rustc_middle::ty::list::List<rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>> as rustc_middle::ty::fold::TypeFoldable>::super_fold_with<rustc_infer::infer::canonical::canonicalizer::Canonicalizer>::{closure
     1,656 ( 0.33%)  ???:rustc_middle::ty::util::fold_list::<rustc_infer::infer::canonical::canonicalizer::Canonicalizer, rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>, <&rustc_middle::ty::list::List<rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>> as rustc_middle::ty::fold::TypeFoldable>::try_super_fold_with<rustc_infer::infer::canonical::canonicalizer::Canonicalizer>::{closure
    -1,646 (-0.33%)  ???:<rustc_middle::ty::sty::ProjectionTy>::trait_ref
    -1,515 (-0.30%)  ???:llvm::StringRef::find(llvm::StringRef, unsigned long) const
     1,454 ( 0.29%)  ???:<rustc_infer::infer::sub::Sub as rustc_middle::ty::relate::TypeRelation>::relate_with_variance::<&rustc_middle::ty::TyS>
    -1,368 (-0.27%)  ???:<rustc_infer::infer::canonical::canonicalizer::Canonicalizer as rustc_middle::ty::fold::TypeFolder>::fold_binder::<rustc_middle::ty::sty::ExistentialPredicate>
     1,360 ( 0.27%)  ???:rustc_middle::ty::util::fold_list::<rustc_middle::ty::fold::BoundVarReplacer, rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>, <&rustc_middle::ty::list::List<rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>> as rustc_middle::ty::fold::TypeFoldable>::try_super_fold_with<rustc_middle::ty::fold::BoundVarReplacer>::{closure
    -1,352 (-0.27%)  ???:rustc_middle::ty::util::fold_list::<rustc_middle::ty::fold::BoundVarReplacer, rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>, <&rustc_middle::ty::list::List<rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>> as rustc_middle::ty::fold::TypeFoldable>::super_fold_with<rustc_middle::ty::fold::BoundVarReplacer>::{closure
     1,344 ( 0.27%)  ???:<rustc_infer::infer::canonical::canonicalizer::Canonicalizer as rustc_middle::ty::fold::FallibleTypeFolder>::try_fold_binder::<rustc_middle::ty::sty::ExistentialPredicate>
     1,300 ( 0.26%)  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_infer::infer::freshen::TypeFreshener>
     1,300 ( 0.26%)  ???:<object::read::elf::file::ElfFile<object::elf::FileHeader64<object::endian::Endianness>>>::parse
    -1,300 (-0.26%)  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_middle::ty::fold::TypeFoldable>::fold_with::<rustc_infer::infer::freshen::TypeFreshener>
     1,238 ( 0.24%)  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::Predicate> as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_middle::ty::fold::BoundVarReplacer>
    -1,238 (-0.24%)  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::Predicate> as rustc_middle::ty::fold::TypeFoldable>::fold_with::<rustc_middle::ty::fold::BoundVarReplacer>
     1,220 ( 0.24%)  ???:<rustc_infer::infer::resolve::OpportunisticVarResolver as rustc_middle::ty::fold::FallibleTypeFolder>::try_fold_ty
     1,183 ( 0.23%)  ???:<rustc_infer::infer::InferCtxt>::subtype_predicate
     1,136 ( 0.22%)  ???:<rustc_infer::infer::freshen::TypeFreshener as rustc_middle::ty::fold::FallibleTypeFolder>::try_fold_ty
     1,092 ( 0.22%)  ???:rustc_middle::ty::util::fold_list::<rustc_infer::infer::resolve::OpportunisticVarResolver, rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>, <&rustc_middle::ty::list::List<rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>> as rustc_middle::ty::fold::TypeFoldable>::try_super_fold_with<rustc_infer::infer::resolve::OpportunisticVarResolver>::{closure
    -1,071 (-0.21%)  ???:<rustc_middle::ty::context::TyCtxt>::replace_late_bound_regions::<rustc_middle::ty::PredicateKind, <rustc_middle::ty::context::TyCtxt>::anonymize_late_bound_regions<rustc_middle::ty::PredicateKind>::{closure
    -1,040 (-0.21%)  ???:<&mut rustc_infer::infer::undo_log::InferCtxtUndoLogs as ena::undo_log::UndoLogs<rustc_data_structures::snapshot_map::UndoLog<rustc_infer::traits::project::ProjectionCacheKey, rustc_infer::traits::project::ProjectionCacheEntry>>>::push
     1,025 ( 0.20%)  ???:<hashbrown::map::RawEntryBuilderMut<rustc_middle::ty::context::Interned<rustc_middle::ty::PredicateInner>, (), core::hash::BuildHasherDefault<rustc_hash::FxHasher>>>::from_hash::<hashbrown::map::equivalent<rustc_middle::ty::sty::Binder<rustc_middle::ty::PredicateKind>, rustc_middle::ty::context::Interned<rustc_middle::ty::PredicateInner>>::{closure
    -1,014 (-0.20%)  ???:rustc_middle::ty::util::fold_list::<rustc_middle::ty::fold::RegionFolder, rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>, <&rustc_middle::ty::list::List<rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>> as rustc_middle::ty::fold::TypeFoldable>::super_fold_with<rustc_middle::ty::fold::RegionFolder>::{closure
      -990 (-0.20%)  ???:<&rustc_middle::ty::TyS as rustc_middle::ty::fold::TypeFoldable>::visit_with::<<rustc_middle::ty::context::TyCtxt>::any_free_region_meets::RegionVisitor<<rustc_middle::ty::context::TyCtxt>::all_free_regions_meet<&rustc_middle::ty::TyS, rustc_borrowck::type_check::liveness::compute_live_locals::{closure
      -970 (-0.19%)  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_middle::ty::fold::TypeFoldable>::fold_with::<rustc_infer::infer::resolve::OpportunisticVarResolver>
      -966 (-0.19%)  ???:rustc_middle::ty::util::fold_list::<rustc_infer::infer::resolve::OpportunisticVarResolver, rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>, <&rustc_middle::ty::list::List<rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>> as rustc_middle::ty::fold::TypeFoldable>::super_fold_with<rustc_infer::infer::resolve::OpportunisticVarResolver>::{closure
      -962 (-0.19%)  ???:<&rustc_middle::ty::TyS as rustc_middle::ty::fold::TypeFoldable>::super_fold_with::<rustc_trait_selection::traits::query::normalize::QueryNormalizer>
       962 ( 0.19%)  ???:<&rustc_middle::ty::TyS as rustc_middle::ty::fold::TypeFoldable>::try_super_fold_with::<rustc_trait_selection::traits::query::normalize::QueryNormalizer>
      -944 (-0.19%)  ???:<rustc_middle::ty::PredicateKind as rustc_middle::ty::fold::TypeFoldable>::fold_with::<rustc_trait_selection::traits::query::normalize::QueryNormalizer>
       944 ( 0.19%)  ???:<rustc_middle::ty::PredicateKind as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_trait_selection::traits::query::normalize::QueryNormalizer>
       890 ( 0.18%)  ???:<rustc_middle::ty::Predicate as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_middle::ty::fold::BoundVarReplacer>
      -890 (-0.18%)  ???:<rustc_middle::ty::Predicate as rustc_middle::ty::fold::TypeFoldable>::fold_with::<rustc_middle::ty::fold::BoundVarReplacer>
       833 ( 0.16%)  ???:<core::slice::iter::Iter<rustc_middle::ty::FieldDef> as core::iter::traits::iterator::Iterator>::try_fold::<alloc::vec::Vec<&rustc_middle::ty::TyS>, &mut core::iter::adapters::map::map_try_fold<&rustc_middle::ty::FieldDef, &rustc_middle::ty::TyS, alloc::vec::Vec<&rustc_middle::ty::TyS>, core::result::Result<alloc::vec::Vec<&rustc_middle::ty::TyS>, rustc_middle::ty::util::AlwaysRequiresDrop>, rustc_ty_utils::needs_drop::drop_tys_helper<rustc_ty_utils::needs_drop::needs_drop_raw::{closure
       820 ( 0.16%)  ???:split(llvm::StringRef, char, std::pair<llvm::StringRef, llvm::StringRef>&)
       816 ( 0.16%)  ???:<rustc_middle::ty::context::TyCtxt>::replace_escaping_bound_vars::<rustc_middle::ty::ParamEnvAnd<&rustc_middle::ty::TyS>, rustc_infer::infer::canonical::substitute::substitute_value<rustc_middle::ty::ParamEnvAnd<&rustc_middle::ty::TyS>>::{closure
       792 ( 0.16%)  ???:rustc_middle::ty::util::fold_list::<rustc_middle::ty::fold::RegionFolder, rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>, <&rustc_middle::ty::list::List<rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>> as rustc_middle::ty::fold::TypeFoldable>::try_super_fold_with<rustc_middle::ty::fold::RegionFolder>::{closure
       783 ( 0.15%)  ???:<rustc_infer::infer::combine::Generalizer as rustc_middle::ty::relate::TypeRelation>::tys
       773 ( 0.15%)  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_trait_selection::traits::query::normalize::QueryNormalizer>
      -773 (-0.15%)  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_middle::ty::fold::TypeFoldable>::fold_with::<rustc_trait_selection::traits::query::normalize::QueryNormalizer>
       768 ( 0.15%)  ???:<rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::FallibleTypeFolder>::try_fold_binder::<rustc_middle::ty::PredicateKind>
      -768 (-0.15%)  ???:<rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_binder::<rustc_middle::ty::PredicateKind>
       720 ( 0.14%)  ???:<&mut rustc_infer::infer::undo_log::InferCtxtUndoLogs as ena::undo_log::UndoLogs<ena::snapshot_vec::UndoLog<ena::unify::backing_vec::Delegate<rustc_infer::infer::type_variable::TyVidEqKey>>>>::push
       692 ( 0.14%)  ???:<rustc_middle::traits::query::type_op::ProvePredicate as rustc_trait_selection::traits::query::type_op::QueryTypeOp>::fully_perform_into
      -680 (-0.13%)  ???:<&rustc_middle::ty::TyS as rustc_middle::ty::fold::TypeFoldable>::visit_with::<<rustc_middle::ty::context::TyCtxt>::any_free_region_meets::RegionVisitor<<rustc_middle::ty::context::TyCtxt>::for_each_free_region<&rustc_middle::ty::TyS, <rustc_borrowck::type_check::TypeVerifier as rustc_middle::mir::visit::Visitor>::visit_constant::{closure
       678 ( 0.13%)  ???:<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>>, alloc::vec::into_iter::IntoIter<rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>>>, <&rustc_middle::ty::list::List<rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>> as rustc_middle::ty::relate::Relate>::relate<rustc_infer::infer::sub::Sub>::{closure
       672 ( 0.13%)  ???:<rustc_middle::ty::fold::RegionFolder as rustc_middle::ty::fold::FallibleTypeFolder>::try_fold_binder::<rustc_middle::ty::sty::ExistentialPredicate>
       667 ( 0.13%)  ???:<&rustc_middle::ty::sty::RegionKind as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_infer::infer::canonical::canonicalizer::Canonicalizer>
      -630 (-0.12%)  ???:<core::result::Result<rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>, rustc_middle::ty::error::TypeError> as rustc_middle::ty::context::InternIteratorElement<rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>, &rustc_middle::ty::list::List<rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>>>>::intern_with::<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>>, alloc::vec::into_iter::IntoIter<rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>>>, <&rustc_middle::ty::list::List<rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>> as rustc_middle::ty::relate::Relate>::relate<rustc_infer::infer::sub::Sub>::{closure
       628 ( 0.12%)  ./elf/../sysdeps/x86_64/dl-machine.h:_dl_relocate_object
      -606 (-0.12%)  ???:<rustc_middle::ty::context::TyCtxt>::mk_substs_trait
      -600 (-0.12%)  ???:<object::elf::FileHeader64<object::endian::Endianness> as object::read::elf::file::FileHeader>::program_headers::<&[u8]>
      -584 (-0.12%)  ???:<rustc_span::source_map::SourceMap>::new_imported_source_file
       583 ( 0.12%)  ???:<rustc_trait_selection::traits::select::SelectionContext>::evaluate_trait_predicate_recursively
      -576 (-0.11%)  ???:<&rustc_middle::ty::TyS as rustc_middle::ty::fold::TypeFoldable>::fold_with::<rustc_middle::ty::erase_regions::RegionEraserVisitor>
       567 ( 0.11%)  ???:rustc_middle::ty::util::fold_list::<rustc_trait_selection::traits::query::normalize::QueryNormalizer, rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>, <&rustc_middle::ty::list::List<rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>> as rustc_middle::ty::fold::TypeFoldable>::try_super_fold_with<rustc_trait_selection::traits::query::normalize::QueryNormalizer>::{closure
      -567 (-0.11%)  ???:rustc_middle::ty::util::fold_list::<rustc_trait_selection::traits::query::normalize::QueryNormalizer, rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>, <&rustc_middle::ty::list::List<rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>> as rustc_middle::ty::fold::TypeFoldable>::super_fold_with<rustc_trait_selection::traits::query::normalize::QueryNormalizer>::{closure
       560 ( 0.11%)  ???:<rustc_middle::ty::erase_regions::RegionEraserVisitor as rustc_middle::ty::fold::FallibleTypeFolder>::try_fold_ty
      -527 (-0.10%)  ???:rustc_middle::ty::util::fold_list::<rustc_middle::ty::fold::BoundVarReplacer, rustc_middle::ty::Predicate, <&rustc_middle::ty::list::List<rustc_middle::ty::Predicate> as rustc_middle::ty::fold::TypeFoldable>::super_fold_with<rustc_middle::ty::fold::BoundVarReplacer>::{closure
       527 ( 0.10%)  ???:rustc_middle::ty::util::fold_list::<rustc_middle::ty::fold::BoundVarReplacer, rustc_middle::ty::Predicate, <&rustc_middle::ty::list::List<rustc_middle::ty::Predicate> as rustc_middle::ty::fold::TypeFoldable>::try_super_fold_with<rustc_middle::ty::fold::BoundVarReplacer>::{closure
      -520 (-0.10%)  ./string/../sysdeps/x86_64/multiarch/memmove-vec-unaligned-erms.S:memcpy@GLIBC_2.2.5

--------------------------------------------------------------------------------
The following files chosen for auto-annotation could not be found:
--------------------------------------------------------------------------------
  ./elf/../sysdeps/x86_64/dl-machine.h
  ./elf/dl-lookup.c
  ./string/../sysdeps/x86_64/multiarch/memmove-vec-unaligned-erms.S
