text
--------------------------------------------------------------------------------
Files compared:   results/cgfilt-49b9cc5139dd4d11ef78dc08c1f9170de5b1ca39-diesel-1.4.8-Check-Full; results/cgfilt-3c471c0bbf60528fb31cc524fbb5a51c124d6fae-diesel-1.4.8-Check-Full
Command:          /usr/local/rustup/toolchains/49b9cc5139dd4d11ef78dc08c1f9170de5b1ca39/bin/rustc --crate-name diesel src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata -C embed-bitcode=no -C debuginfo=2 --cfg feature="32-column-tables" --cfg feature="default" --cfg feature="with-deprecated" -C metadata=06cab02550f1137d -C extra-filename=-06cab02550f1137d --out-dir /tmp/.tmp4K5BqG/target/debug/deps -L dependency=/tmp/.tmp4K5BqG/target/debug/deps --extern byteorder=/tmp/.tmp4K5BqG/target/debug/deps/libbyteorder-19cac908f6232626.rmeta --extern diesel_derives=/tmp/.tmp4K5BqG/target/debug/deps/libdiesel_derives-2b9e82abad3a8359.so -Adeprecated -Aunknown-lints -Zincremental-verify-ich; /usr/local/rustup/toolchains/3c471c0bbf60528fb31cc524fbb5a51c124d6fae/bin/rustc --crate-name diesel src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata -C embed-bitcode=no -C debuginfo=2 --cfg feature="32-column-tables" --cfg feature="default" --cfg feature="with-deprecated" -C metadata=06cab02550f1137d -C extra-filename=-06cab02550f1137d --out-dir /tmp/.tmpYlEaAF/target/debug/deps -L dependency=/tmp/.tmpYlEaAF/target/debug/deps --extern byteorder=/tmp/.tmpYlEaAF/target/debug/deps/libbyteorder-19cac908f6232626.rmeta --extern diesel_derives=/tmp/.tmpYlEaAF/target/debug/deps/libdiesel_derives-2b9e82abad3a8359.so -Adeprecated -Aunknown-lints -Zincremental-verify-ich
Data file:        results/cgfilt-diff-49b9cc5139dd4d11ef78dc08c1f9170de5b1ca39-3c471c0bbf60528fb31cc524fbb5a51c124d6fae-diesel-1.4.8-Check-Full
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
437,023,572  PROGRAM TOTALS

--------------------------------------------------------------------------------
Ir              file:function
--------------------------------------------------------------------------------
 1,227,932,789  ???:rustc_middle::ty::util::fold_list::<rustc_infer::infer::resolve::OpportunisticVarResolver, rustc_middle::ty::Predicate, <rustc_middle::ty::context::TyCtxt>::intern_predicates>
-1,159,230,060  ???:rustc_middle::ty::util::fold_list::<rustc_infer::infer::resolve::OpportunisticVarResolver, rustc_middle::ty::Predicate, <&rustc_middle::ty::list::List<rustc_middle::ty::Predicate> as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with<rustc_infer::infer::resolve::OpportunisticVarResolver>::{closure
   524,813,966  ???:rustc_middle::ty::util::fold_list::<rustc_infer::infer::canonical::canonicalizer::Canonicalizer, rustc_middle::ty::Predicate, <rustc_middle::ty::context::TyCtxt>::intern_predicates>
  -509,812,422  ???:rustc_middle::ty::util::fold_list::<rustc_infer::infer::canonical::canonicalizer::Canonicalizer, rustc_middle::ty::Predicate, <&rustc_middle::ty::list::List<rustc_middle::ty::Predicate> as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with<rustc_infer::infer::canonical::canonicalizer::Canonicalizer>::{closure
  -289,253,511  ???:rustc_trait_selection::traits::wf::obligations
   284,229,133  ???:<rustc_trait_selection::traits::fulfill::FulfillProcessor as rustc_data_structures::obligation_forest::ObligationProcessor>::process_obligation
   185,752,125  ???:<alloc::vec::Vec<rustc_middle::ty::sty::Binder<rustc_middle::ty::OutlivesPredicate<rustc_middle::ty::Ty, rustc_middle::ty::sty::Region>>> as alloc::vec::spec_from_iter::SpecFromIter<rustc_middle::ty::sty::Binder<rustc_middle::ty::OutlivesPredicate<rustc_middle::ty::Ty, rustc_middle::ty::sty::Region>>, core::iter::adapters::inspect::Inspect<core::iter::adapters::chain::Chain<core::iter::adapters::filter::Filter<core::iter::adapters::filter_map::FilterMap<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_middle::ty::Predicate>>, <rustc_infer::infer::outlives::verify::VerifyBoundCx>::collect_outlives_from_predicate_list<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_middle::ty::Predicate>>>::{closure
  -169,264,498  ???:<rustc_infer::infer::outlives::verify::VerifyBoundCx>::declared_generic_bounds_from_env_for_erased_ty
  -141,212,288  ???:<rustc_middle::ty::ParamEnvAnd<rustc_middle::traits::query::type_op::ProvePredicate> as rustc_trait_selection::traits::query::type_op::TypeOp>::fully_perform
   137,420,480  ???:<rustc_middle::traits::query::type_op::ProvePredicate as rustc_trait_selection::traits::query::type_op::QueryTypeOp>::fully_perform_into
    71,558,014  ???:<rustc_trait_selection::traits::fulfill::FulfillmentContext as rustc_infer::traits::engine::TraitEngine>::register_predicate_obligation
    63,630,878  ???:<rustc_hir_typeck::inherited::Inherited>::register_predicate
    61,183,109  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::Predicate> as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with::<rustc_infer::infer::resolve::OpportunisticVarResolver>
   -38,021,962  ???:<alloc::collections::btree::node::NodeRef<alloc::collections::btree::node::marker::Mut, rustc_infer::infer::region_constraints::Constraint, rustc_infer::infer::SubregionOrigin, alloc::collections::btree::node::marker::LeafOrInternal>>::search_tree::<rustc_infer::infer::region_constraints::Constraint>
   -37,285,339  ???:<rustc_hir_typeck::coercion::Coerce>::coerce
    34,101,182  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::Predicate> as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with::<rustc_infer::infer::canonical::canonicalizer::Canonicalizer>
    30,859,434  ???:<rustc_infer::infer::resolve::OpportunisticVarResolver as rustc_type_ir::fold::FallibleTypeFolder<rustc_middle::ty::context::TyCtxt>>::try_fold_ty
   -30,361,335  ???:<rustc_middle::ty::Ty as rustc_type_ir::fold::TypeSuperFoldable<rustc_middle::ty::context::TyCtxt>>::super_fold_with::<rustc_infer::infer::freshen::TypeFreshener>
    29,637,374  ???:<rustc_infer::infer::InferCtxt>::rollback_to
    29,162,767  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with::<rustc_middle::ty::subst::SubstFolder>
   -25,169,777  ???:<rustc_middle::ty::Ty as rustc_type_ir::fold::TypeSuperFoldable<rustc_middle::ty::context::TyCtxt>>::super_fold_with::<rustc_middle::ty::subst::SubstFolder>
    22,210,454  ???:<rustc_hir_typeck::coercion::Coerce>::coerce_unsized
    22,089,535  ???:<rustc_infer::infer::InferCtxt>::commit_if_ok::<rustc_infer::infer::InferOk<(alloc::vec::Vec<rustc_middle::ty::adjustment::Adjustment>, rustc_middle::ty::Ty)>, rustc_middle::ty::error::TypeError, <rustc_hir_typeck::coercion::Coerce>::coerce::{closure
    19,446,850  ???:rustc_middle::ty::util::fold_list::<rustc_infer::infer::freshen::TypeFreshener, rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>, <rustc_middle::ty::context::TyCtxt>::intern_poly_existential_predicates>
    18,905,208  ???:<rustc_middle::ty::context::CtxtInterners>::intern_ty
    18,395,713  ???:<rustc_infer::traits::Obligation<rustc_middle::ty::Predicate> as rustc_middle::ty::visit::TypeVisitableExt>::has_escaping_bound_vars
   -18,003,969  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::Predicate> as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with::<rustc_middle::ty::fold::BoundVarReplacer<rustc_middle::ty::fold::FnMutDelegate>>
    17,824,140  ???:rustc_middle::ty::util::fold_list::<rustc_middle::ty::fold::BoundVarReplacer<rustc_middle::ty::fold::FnMutDelegate>, rustc_middle::ty::Predicate, <rustc_middle::ty::context::TyCtxt>::intern_predicates>
    17,751,602  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with::<rustc_infer::infer::freshen::TypeFreshener>
   -16,118,419  ???:<rustc_infer::infer::at::At>::eq::<rustc_middle::ty::sty::TraitRef>
   -15,617,477  ???:<dyn rustc_infer::traits::engine::TraitEngine as rustc_infer::traits::engine::TraitEngineExt>::register_predicate_obligations::<alloc::vec::Vec<rustc_infer::traits::Obligation<rustc_middle::ty::Predicate>>>
    14,870,044  ???:<rustc_infer::infer::InferCtxt>::probe::<(), <rustc_trait_selection::traits::select::SelectionContext>::assemble_candidates_from_impls::{closure
   -14,554,438  ???:<rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::query::evaluate_obligation::InferCtxtExt>::evaluate_obligation
    14,170,323  ???:<rustc_infer::infer::canonical::canonicalizer::Canonicalizer>::canonicalize::<rustc_middle::ty::ParamEnvAnd<rustc_middle::ty::Predicate>>
   -14,084,264  ???:<rustc_middle::ty::context::TyCtxt>::for_each_relevant_impl::<<rustc_trait_selection::traits::select::SelectionContext>::assemble_candidates_from_impls::{closure
    13,817,082  ???:<rustc_trait_selection::traits::select::SelectionContext>::match_impl
   -13,733,345  ???:<rustc_borrowck::constraint_generation::ConstraintGeneration as rustc_middle::mir::visit::Visitor>::visit_basic_block_data
    13,624,439  ???:<rustc_middle::traits::ObligationCause as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with::<rustc_infer::infer::resolve::OpportunisticVarResolver>
    12,867,617  ???:<rustc_middle::ty::Ty as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with::<rustc_middle::ty::subst::SubstFolder>
   -12,728,173  ???:<rustc_middle::ty::Ty as rustc_type_ir::fold::TypeSuperFoldable<rustc_middle::ty::context::TyCtxt>>::super_fold_with::<rustc_infer::infer::resolve::OpportunisticVarResolver>
    12,322,522  ???:<rustc_borrowck::type_check::TypeChecker>::typeck_mir
   -12,170,315  ???:<alloc::vec::Vec<rustc_middle::ty::Predicate> as alloc::vec::spec_from_iter::SpecFromIter<rustc_middle::ty::Predicate, core::iter::adapters::GenericShunt<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<rustc_middle::ty::Predicate>, <alloc::vec::Vec<rustc_middle::ty::Predicate> as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with<rustc_infer::infer::resolve::OpportunisticVarResolver>::{closure
    11,863,451  ???:<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<rustc_middle::ty::Predicate>, <alloc::vec::Vec<rustc_middle::ty::Predicate> as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with<rustc_infer::infer::resolve::OpportunisticVarResolver>::{closure
   -11,165,878  ???:<rustc_trait_selection::traits::engine::ObligationCtxt>::register_obligation
   -10,810,952  ???:<rustc_middle::ty::context::CtxtInterners>::intern_predicate
    10,791,603  ???:rustc_borrowck::constraint_generation::generate_constraints
   -10,201,541  ???:<rustc_hir_typeck::inherited::Inherited>::register_infer_ok_obligations::<rustc_middle::ty::sty::FnSig>
     9,686,045  ???:<rustc_middle::traits::ObligationCauseCode as rustc_type_ir::visit::TypeVisitable<rustc_middle::ty::context::TyCtxt>>::visit_with::<rustc_middle::ty::visit::HasTypeFlagsVisitor>
     9,140,190  ???:<alloc::collections::btree::map::BTreeMap<rustc_infer::infer::region_constraints::Constraint, rustc_infer::infer::SubregionOrigin>>::entry
     8,903,343  ???:<rustc_middle::traits::ObligationCauseCode as rustc_type_ir::visit::TypeVisitable<rustc_middle::ty::context::TyCtxt>>::visit_with::<rustc_middle::ty::visit::HasEscapingVarsVisitor>
     8,805,198  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>> as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with::<rustc_infer::infer::canonical::canonicalizer::Canonicalizer>
    -8,536,136  ???:<rustc_middle::ty::ParamEnvAnd<rustc_trait_selection::traits::query::type_op::implied_outlives_bounds::ImpliedOutlivesBounds> as rustc_trait_selection::traits::query::type_op::TypeOp>::fully_perform
    -8,321,525  ???:<rustc_hir_typeck::inherited::Inherited>::register_predicates::<alloc::vec::Vec<rustc_infer::traits::Obligation<rustc_middle::ty::Predicate>>>
     7,814,285  ???:<rustc_hir_typeck::fn_ctxt::FnCtxt>::require_type_is_sized
     7,812,282  ???:<rustc_trait_selection::traits::query::type_op::implied_outlives_bounds::ImpliedOutlivesBounds as rustc_trait_selection::traits::query::type_op::QueryTypeOp>::fully_perform_into
     7,492,161  ???:<rustc_middle::ty::subst::SubstFolder as rustc_type_ir::fold::FallibleTypeFolder<rustc_middle::ty::context::TyCtxt>>::try_fold_ty
     6,800,151  ???:<rustc_trait_selection::traits::select::SelectionContext>::select
     6,637,157  ???:rustc_middle::ty::util::fold_list::<rustc_middle::ty::fold::BoundVarReplacer<rustc_middle::ty::fold::FnMutDelegate>, rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>, <rustc_middle::ty::context::TyCtxt>::intern_poly_existential_predicates>
     6,509,628  ???:<rustc_infer::infer::canonical::canonicalizer::Canonicalizer>::canonicalize::<rustc_middle::infer::canonical::QueryResponse<()>>
    -6,312,188  ???:<rustc_trait_selection::traits::engine::ObligationCtxt>::normalize::<rustc_middle::ty::Ty>
    -6,277,826  ???:<rustc_infer::infer::canonical::canonicalizer::Canonicalizer as rustc_type_ir::fold::TypeFolder<rustc_middle::ty::context::TyCtxt>>::fold_ty
    -6,266,336  ???:<rustc_middle::ty::generics::GenericPredicates>::instantiate_into
     5,893,563  ???:<alloc::vec::Vec<rustc_infer::traits::Obligation<rustc_middle::ty::Predicate>> as alloc::vec::spec_from_iter::SpecFromIter<rustc_infer::traits::Obligation<rustc_middle::ty::Predicate>, core::iter::adapters::filter::Filter<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<rustc_middle::ty::Predicate>, alloc::vec::into_iter::IntoIter<rustc_span::span_encoding::Span>>, core::iter::adapters::rev::Rev<alloc::vec::into_iter::IntoIter<rustc_span::def_id::DefId>>>, <rustc_trait_selection::traits::wf::WfPredicates>::nominal_obligations_inner::{closure
    -5,746,362  ???:<rustc_infer::infer::InferCtxt>::canonicalize_query_keep_static::<rustc_middle::ty::ParamEnvAnd<rustc_middle::traits::query::type_op::Normalize<rustc_middle::ty::sty::FnSig>>>
    -5,563,484  ???:<rustc_infer::infer::InferCtxt>::probe::<core::result::Result<rustc_middle::traits::select::EvaluationResult, rustc_middle::traits::select::OverflowError>, <rustc_trait_selection::traits::select::SelectionContext>::evaluation_probe<<rustc_trait_selection::traits::select::SelectionContext>::evaluate_root_obligation::{closure
    -5,512,034  ???:<rustc_middle::ty::Predicate as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::fold_with::<rustc_infer::infer::resolve::OpportunisticVarResolver>
    -5,301,931  ???:<rustc_infer::infer::at::At as rustc_trait_selection::traits::project::NormalizeExt>::normalize::<rustc_middle::ty::InstantiatedPredicates>
     5,198,100  ???:<rustc_middle::ty::subst::GenericArg as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with::<rustc_middle::ty::subst::SubstFolder>
     5,174,715  ???:<rustc_borrowck::type_check::TypeChecker>::normalize_and_prove_instantiated_predicates
    -5,102,561  ???:<rustc_middle::ty::Ty as rustc_serialize::serialize::Encodable<rustc_metadata::rmeta::encoder::EncodeContext>>::encode
     5,067,697  ???:<rustc_middle::ty::sty::Binder<rustc_middle::ty::PredicateKind> as rustc_serialize::serialize::Encodable<rustc_metadata::rmeta::encoder::EncodeContext>>::encode
    -5,004,140  ???:<rustc_infer::infer::InferCtxt>::make_canonicalized_query_response::<()>
     4,898,196  ???:rustc_middle::ty::util::fold_list::<rustc_middle::ty::subst::SubstFolder, rustc_middle::ty::subst::GenericArg, <rustc_middle::ty::context::TyCtxt>::mk_substs>
    -4,698,699  ???:<rustc_metadata::rmeta::encoder::EncodeContext>::encode_def_ids
    -4,451,021  ???:<rustc_middle::ty::ParamEnvAnd<rustc_middle::traits::query::type_op::Normalize<rustc_middle::ty::Ty>> as rustc_trait_selection::traits::query::type_op::TypeOp>::fully_perform
     4,256,134  ???:<rustc_trait_selection::traits::select::SelectionContext>::confirm_candidate
    -4,132,065  ???:<rustc_infer::infer::at::At as rustc_trait_selection::traits::project::NormalizeExt>::normalize::<(rustc_middle::ty::sty::FnSig, rustc_middle::ty::InstantiatedPredicates)>
     4,104,562  ???:<rustc_middle::ty::Ty as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with::<rustc_middle::ty::fold::RegionFolder>
     4,056,805  ???:rustc_traits::evaluate_obligation::evaluate_obligation
     4,036,391  ???:rustc_middle::ty::codec::encode_with_shorthand::<rustc_metadata::rmeta::encoder::EncodeContext, rustc_middle::ty::Ty, <rustc_metadata::rmeta::encoder::EncodeContext as rustc_type_ir::codec::TyEncoder>::type_shorthands>
    -3,789,360  ???:<rustc_infer::infer::InferCtxt>::probe::<rustc_hir_typeck::method::probe::ProbeResult, <rustc_hir_typeck::method::probe::ProbeContext>::consider_probe::{closure
     3,687,941  ???:<rustc_trait_selection::traits::select::SelectionContext>::match_projection_projections
    -3,629,898  ???:rustc_trait_selection::traits::project::opt_normalize_projection_type
     3,557,571  ???:<rustc_trait_selection::traits::engine::ObligationCtxt>::register_obligations::<alloc::vec::Vec<rustc_infer::traits::Obligation<rustc_middle::ty::Predicate>>>
    -3,462,458  ???:<rustc_trait_selection::traits::select::SelectionContext>::evaluate_stack
    -3,388,539  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::Ty> as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with::<rustc_middle::ty::subst::SubstFolder>
    -3,334,557  ???:<rustc_middle::ty::Ty as rustc_type_ir::fold::TypeSuperFoldable<rustc_middle::ty::context::TyCtxt>>::super_fold_with::<rustc_middle::ty::fold::BoundVarReplacer<rustc_middle::ty::fold::FnMutDelegate>>
    -3,309,935  ???:<rustc_infer::infer::InferCtxt>::canonicalize_query_keep_static::<rustc_middle::ty::ParamEnvAnd<rustc_middle::traits::query::type_op::Normalize<rustc_middle::ty::Predicate>>>
    -3,278,252  ???:rustc_middle::ty::util::fold_list::<rustc_infer::infer::resolve::OpportunisticVarResolver, rustc_middle::ty::Ty, <&rustc_middle::ty::list::List<rustc_middle::ty::Ty> as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with<rustc_infer::infer::resolve::OpportunisticVarResolver>::{closure
     3,269,289  ???:<rustc_infer::infer::outlives::verify::VerifyBoundCx>::param_bound
     3,263,487  ???:<alloc::vec::Vec<rustc_infer::infer::region_constraints::VerifyBound> as alloc::vec::spec_from_iter::SpecFromIter<rustc_infer::infer::region_constraints::VerifyBound, core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<rustc_middle::ty::sty::Binder<rustc_middle::ty::OutlivesPredicate<rustc_middle::ty::Ty, rustc_middle::ty::sty::Region>>>, <rustc_infer::infer::outlives::verify::VerifyBoundCx>::alias_bound::{closure
     3,190,085  ???:<rustc_middle::traits::ImplDerivedObligationCause as rustc_type_ir::visit::TypeVisitable<rustc_middle::ty::context::TyCtxt>>::visit_with::<rustc_middle::ty::visit::HasTypeFlagsVisitor>
     3,007,617  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with::<rustc_infer::infer::resolve::FullTypeResolver>
     2,921,857  ???:rustc_trait_selection::traits::project::normalize_with_depth::<(rustc_middle::ty::sty::FnSig, rustc_middle::ty::InstantiatedPredicates)>
     2,834,228  ???:<rustc_middle::ty::fold::BoundVarReplacer<rustc_middle::ty::fold::FnMutDelegate> as rustc_type_ir::fold::FallibleTypeFolder<rustc_middle::ty::context::TyCtxt>>::try_fold_ty
    -2,745,593  ???:rustc_middle::ty::util::fold_list::<rustc_middle::ty::subst::SubstFolder, rustc_middle::ty::Ty, <&rustc_middle::ty::list::List<rustc_middle::ty::Ty> as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with<rustc_middle::ty::subst::SubstFolder>::{closure
    -2,740,613  ???:<rustc_infer::infer::InferCtxt>::commit_if_ok::<rustc_infer::infer::InferOk<()>, rustc_middle::ty::error::TypeError, <rustc_infer::infer::at::Trace>::sub<rustc_middle::ty::sty::AliasTy>::{closure
     2,652,895  ???:<rustc_borrowck::type_check::TypeVerifier as rustc_middle::mir::visit::Visitor>::visit_place
    -2,627,576  ???:<rustc_middle::ty::fold::RegionFolder as rustc_type_ir::fold::FallibleTypeFolder<rustc_middle::ty::context::TyCtxt>>::try_fold_ty
     2,561,933  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>> as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with::<rustc_infer::infer::freshen::TypeFreshener>
     2,528,955  ???:rustc_middle::ty::relate::super_relate_tys::<rustc_infer::infer::sub::Sub>
    -2,502,569  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>> as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with::<rustc_middle::ty::fold::BoundVarReplacer<rustc_middle::ty::fold::FnMutDelegate>>
     2,424,418  ???:<rustc_middle::ty::sty::Region as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with::<rustc_middle::ty::fold::BoundVarReplacer<<rustc_infer::infer::InferCtxt>::instantiate_binder_with_fresh_vars::ToFreshVars>>
     2,421,653  ???:<alloc::rc::Rc<rustc_middle::traits::ObligationCauseCode> as core::ops::drop::Drop>::drop
    -2,419,685  ???:rustc_middle::ty::util::fold_list::<rustc_trait_selection::traits::project::AssocTypeNormalizer, rustc_middle::ty::subst::GenericArg, <&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with<rustc_trait_selection::traits::project::AssocTypeNormalizer>::{closure
     2,418,812  ???:rustc_middle::ty::util::fold_list::<rustc_trait_selection::traits::project::AssocTypeNormalizer, rustc_middle::ty::subst::GenericArg, <rustc_middle::ty::context::TyCtxt>::mk_substs>
    -2,413,338  ???:<rustc_hir_typeck::inherited::Inherited>::register_infer_ok_obligations::<rustc_middle::ty::InstantiatedPredicates>
    -2,411,468  ???:rustc_middle::ty::util::fold_list::<rustc_middle::ty::subst::SubstFolder, rustc_middle::ty::subst::GenericArg, <&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with<rustc_middle::ty::subst::SubstFolder>::{closure
     2,363,001  ???:<rustc_middle::traits::query::type_op::Normalize<rustc_middle::ty::Ty> as rustc_trait_selection::traits::query::type_op::QueryTypeOp>::fully_perform_into
     2,276,607  ???:rustc_middle::ty::util::fold_list::<rustc_middle::ty::fold::RegionFolder, rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>, <rustc_middle::ty::context::TyCtxt>::intern_poly_existential_predicates>
    -2,241,870  ???:rustc_middle::ty::util::fold_list::<rustc_middle::ty::fold::BoundVarReplacer<rustc_middle::ty::fold::FnMutDelegate>, rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>, <&rustc_middle::ty::list::List<rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>> as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with<rustc_middle::ty::fold::BoundVarReplacer<rustc_middle::ty::fold::FnMutDelegate>>::{closure
     2,211,507  ???:<rustc_privacy::DefIdVisitorSkeleton<rustc_privacy::SearchInterfaceForPrivateItemsVisitor> as rustc_type_ir::visit::TypeVisitor<rustc_middle::ty::context::TyCtxt>>::visit_ty
    -2,204,721  ???:<rustc_middle::ty::Ty as rustc_type_ir::fold::TypeSuperFoldable<rustc_middle::ty::context::TyCtxt>>::super_fold_with::<rustc_middle::ty::fold::BoundVarReplacer<<rustc_infer::infer::InferCtxt>::instantiate_binder_with_fresh_vars::ToFreshVars>>
     2,169,135  ???:rustc_middle::ty::util::fold_list::<rustc_middle::ty::erase_regions::RegionEraserVisitor, rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>, <rustc_middle::ty::context::TyCtxt>::intern_poly_existential_predicates>
    -2,161,795  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with::<rustc_middle::ty::fold::RegionFolder>
     2,087,553  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::Ty> as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with::<rustc_infer::infer::freshen::TypeFreshener>
     2,074,116  ???:rustc_middle::ty::util::fold_list::<rustc_infer::infer::resolve::FullTypeResolver, rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>, <rustc_middle::ty::context::TyCtxt>::intern_poly_existential_predicates>
     2,003,797  ???:rustc_hir_analysis::check::wfcheck::check_where_clauses
     2,001,787  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::Ty> as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with::<rustc_infer::infer::resolve::OpportunisticVarResolver>
    -1,980,013  ???:rustc_middle::ty::util::fold_list::<rustc_infer::infer::canonical::canonicalizer::Canonicalizer, rustc_middle::ty::subst::GenericArg, <&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with<rustc_infer::infer::canonical::canonicalizer::Canonicalizer>::{closure
    -1,976,255  ???:<rustc_middle::ty::Ty as rustc_type_ir::visit::TypeVisitable<rustc_middle::ty::context::TyCtxt>>::visit_with::<rustc_privacy::DefIdVisitorSkeleton<rustc_privacy::SearchInterfaceForPrivateItemsVisitor>>
    -1,968,019  ???:rustc_middle::ty::util::fold_list::<rustc_middle::ty::erase_regions::RegionEraserVisitor, rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>, <&rustc_middle::ty::list::List<rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>> as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with<rustc_middle::ty::erase_regions::RegionEraserVisitor>::{closure
     1,952,080  ???:<rustc_middle::ty::Ty as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with::<rustc_middle::ty::fold::BoundVarReplacer<rustc_middle::ty::fold::FnMutDelegate>>
    -1,885,480  ???:rustc_middle::ty::util::fold_list::<rustc_middle::ty::fold::RegionFolder, rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>, <&rustc_middle::ty::list::List<rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>> as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with<rustc_middle::ty::fold::RegionFolder>::{closure
     1,865,876  ???:<rustc_middle::ty::context::TyCtxt>::replace_escaping_bound_vars_uncached::<rustc_middle::ty::ParamEnvAnd<rustc_middle::traits::query::type_op::AscribeUserType>, rustc_middle::ty::fold::FnMutDelegate>
    -1,857,330  ???:rustc_middle::ty::util::fold_list::<rustc_infer::infer::resolve::FullTypeResolver, rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>, <&rustc_middle::ty::list::List<rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::ExistentialPredicate>> as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with<rustc_infer::infer::resolve::FullTypeResolver>::{closure
    -1,821,233  ???:<rustc_trait_selection::traits::select::SelectionContext>::assemble_candidates
    -1,733,556  ???:rustc_middle::ty::util::fold_list::<rustc_middle::ty::subst::SubstFolder, rustc_middle::ty::Predicate, <&rustc_middle::ty::list::List<rustc_middle::ty::Predicate> as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with<rustc_middle::ty::subst::SubstFolder>::{closure
    -1,719,064  ???:<rustc_infer::infer::outlives::verify::VerifyBoundCx>::bound_from_components
     1,690,357  ???:<rustc_middle::ty::sty::AliasTy as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with::<rustc_middle::ty::subst::SubstFolder>
    -1,686,319  ???:rustc_hir_analysis::check::check::check_impl_items_against_trait
    -1,672,264  ???:<rustc_middle::ty::Ty as rustc_type_ir::fold::TypeSuperFoldable<rustc_middle::ty::context::TyCtxt>>::try_super_fold_with::<rustc_infer::infer::resolve::FullTypeResolver>
     1,663,377  ???:rustc_hir_analysis::check::check::check_mod_item_types
     1,633,230  ???:<rustc_middle::ty::subst::GenericArg as rustc_type_ir::CollectAndApply<rustc_middle::ty::subst::GenericArg, &rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>>>::collect_and_apply::<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_middle::infer::canonical::CanonicalVarInfo>>, <rustc_infer::infer::InferCtxt>::instantiate_canonical_vars<<rustc_infer::infer::InferCtxt>::instantiate_canonical_with_fresh_inference_vars<rustc_middle::ty::ParamEnvAnd<rustc_middle::traits::query::type_op::ProvePredicate>>::{closure
    -1,623,791  ???:rustc_infer::infer::canonical::substitute::substitute_value::<rustc_middle::ty::ParamEnvAnd<rustc_middle::traits::query::type_op::AscribeUserType>>
    -1,558,326  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with::<rustc_middle::ty::erase_regions::RegionEraserVisitor>
    -1,366,345  ???:<rustc_middle::ty::Predicate as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with::<rustc_infer::infer::resolve::OpportunisticVarResolver>
     1,362,173  ???:rustc_hir_analysis::variance::solve::solve_constraints
    -1,362,155  ???:rustc_hir_analysis::variance::crate_variances
    -1,361,833  ???:<rustc_middle::ty::Ty as rustc_type_ir::visit::TypeSuperVisitable<rustc_middle::ty::context::TyCtxt>>::super_visit_with::<<rustc_middle::ty::context::TyCtxt>::any_free_region_meets::RegionVisitor<<rustc_middle::ty::context::TyCtxt>::for_each_free_region<rustc_middle::ty::Ty, <rustc_borrowck::type_check::liveness::trace::LivenessContext>::make_all_regions_live<rustc_middle::ty::Ty>::{closure
     1,345,398  ???:<hashbrown::raw::RawTable<(rustc_span::def_id::DefId, (rustc_span::hygiene::ExpnId, rustc_query_system::dep_graph::graph::DepNodeIndex))>>::reserve_rehash::<hashbrown::map::make_hasher<rustc_span::def_id::DefId, rustc_span::def_id::DefId, (rustc_span::hygiene::ExpnId, rustc_query_system::dep_graph::graph::DepNodeIndex), core::hash::BuildHasherDefault<rustc_hash::FxHasher>>::{closure
    -1,339,158  ???:rustc_borrowck::nll::compute_regions
     1,309,852  ???:<<rustc_middle::ty::context::TyCtxt>::any_free_region_meets::RegionVisitor<<rustc_middle::ty::context::TyCtxt>::for_each_free_region<rustc_middle::ty::Ty, <rustc_borrowck::type_check::TypeVerifier as rustc_middle::mir::visit::Visitor>::visit_constant::{closure
    -1,298,582  ???:rustc_mir_build::thir::pattern::usefulness::compute_match_usefulness
    -1,270,525  ???:<rustc_mir_transform::elaborate_drops::ElaborateDrops as rustc_middle::mir::MirPass>::run_pass
    -1,270,331  ???:<rustc_middle::ty::context::TyCtxt>::def_kind::<rustc_hir::hir_id::OwnerId>
     1,261,073  ???:<rustc_middle::traits::ImplDerivedObligationCause as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with::<rustc_infer::infer::resolve::OpportunisticVarResolver>
    -1,259,398  ???:<hashbrown::raw::RawTable<(rustc_span::def_id::DefId, (rustc_span::span_encoding::Span, rustc_query_system::dep_graph::graph::DepNodeIndex))>>::reserve_rehash::<hashbrown::map::make_hasher<rustc_span::def_id::DefId, rustc_span::def_id::DefId, (rustc_span::span_encoding::Span, rustc_query_system::dep_graph::graph::DepNodeIndex), core::hash::BuildHasherDefault<rustc_hash::FxHasher>>::{closure
     1,258,052  ???:<rustc_middle::ty::context::TyCtxt>::mk_trait_ref::<&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>>
    -1,257,756  ???:<rustc_trait_selection::traits::engine::ObligationCtxt>::normalize::<rustc_middle::ty::Predicate>
     1,254,471  ???:<rustc_borrowck::type_check::TypeVerifier as rustc_middle::mir::visit::Visitor>::visit_operand
    -1,251,246  ???:rustc_hir_analysis::check::wfcheck::check_fn_or_method
     1,224,864  ???:<rustc_lint::late::LateContextAndPass<rustc_lint::BuiltinCombinedModuleLateLintPass> as rustc_hir::intravisit::Visitor>::visit_expr
    -1,215,790  ???:rustc_mir_transform::simplify::simplify_cfg
     1,209,161  ???:<rustc_middle::ty::sty::Region as rustc_type_ir::visit::TypeVisitable<rustc_middle::ty::context::TyCtxt>>::visit_with::<<rustc_middle::ty::context::TyCtxt>::any_free_region_meets::RegionVisitor<<rustc_middle::ty::context::TyCtxt>::all_free_regions_meet<rustc_middle::ty::Ty, rustc_borrowck::type_check::liveness::compute_relevant_live_locals::{closure
     1,208,435  ???:<rustc_infer::infer::type_variable::TypeVariableTable>::equate
     1,203,974  ???:rustc_middle::ty::relate::super_relate_tys::<rustc_infer::infer::combine::Generalizer>
    -1,200,175  ???:<rustc_middle::ty::context::TyCtxt>::mk_param_from_def
     1,185,861  ???:<alloc::vec::Vec<(rustc_mir_build::thir::pattern::usefulness::MatchArm, rustc_mir_build::thir::pattern::usefulness::Reachability)> as alloc::vec::spec_from_iter::SpecFromIter<(rustc_mir_build::thir::pattern::usefulness::MatchArm, rustc_mir_build::thir::pattern::usefulness::Reachability), core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_mir_build::thir::pattern::usefulness::MatchArm>>, rustc_mir_build::thir::pattern::usefulness::compute_match_usefulness::{closure
    -1,183,878  ???:<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_middle::thir::ArmId>>, <rustc_mir_build::build::Builder>::create_match_candidates::{closure
     1,178,426  ???:<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_middle::thir::ArmId>> as core::iter::traits::iterator::Iterator>::fold::<(), core::iter::adapters::map::map_fold<rustc_middle::thir::ArmId, (&rustc_middle::thir::Arm, rustc_mir_build::build::matches::Candidate), (), <rustc_mir_build::build::Builder>::create_match_candidates::{closure
     1,171,896  ???:<rustc_middle::ty::sty::Region as rustc_type_ir::visit::TypeVisitable<rustc_middle::ty::context::TyCtxt>>::visit_with::<<rustc_middle::ty::context::TyCtxt>::any_free_region_meets::RegionVisitor<<rustc_middle::ty::context::TyCtxt>::for_each_free_region<rustc_middle::ty::Ty, <rustc_borrowck::type_check::liveness::trace::LivenessContext>::make_all_regions_live<rustc_middle::ty::Ty>::{closure
    -1,162,826  ???:<rustc_middle::ty::sty::Region as rustc_type_ir::visit::TypeVisitable<rustc_middle::ty::context::TyCtxt>>::visit_with::<<rustc_middle::ty::context::TyCtxt>::any_free_region_meets::RegionVisitor<<rustc_middle::ty::context::TyCtxt>::for_each_free_region<rustc_middle::ty::Ty, <rustc_borrowck::type_check::TypeVerifier as rustc_middle::mir::visit::Visitor>::visit_constant::{closure
     1,162,428  ???:free
    -1,161,611  ???:<rustc_middle::ty::subst::GenericArg as rustc_type_ir::CollectAndApply<rustc_middle::ty::subst::GenericArg, &rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>>>::collect_and_apply::<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_middle::ty::subst::GenericArg>>, <rustc_middle::ty::subst::GenericArg as core::convert::Into<rustc_middle::ty::subst::GenericArg>>::into>, <rustc_middle::ty::context::TyCtxt>::mk_substs_from_iter<core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_middle::ty::subst::GenericArg>>, <rustc_middle::ty::subst::GenericArg as core::convert::Into<rustc_middle::ty::subst::GenericArg>>::into>, rustc_middle::ty::subst::GenericArg>::{closure
     1,157,681  ???:rustc_middle::ty::util::fold_list::<rustc_middle::ty::fold::BoundVarReplacer<rustc_middle::ty::fold::FnMutDelegate>, rustc_middle::ty::subst::GenericArg, <rustc_middle::ty::context::TyCtxt>::mk_substs>
     1,155,636  ???:<rustc_middle::ty::Predicate>::subst_supertrait
    -1,153,498  ???:<rustc_middle::ty::Ty as rustc_type_ir::visit::TypeSuperVisitable<rustc_middle::ty::context::TyCtxt>>::super_visit_with::<<rustc_middle::ty::context::TyCtxt>::any_free_region_meets::RegionVisitor<<rustc_middle::ty::context::TyCtxt>::all_free_regions_meet<rustc_middle::ty::Ty, rustc_borrowck::type_check::liveness::compute_relevant_live_locals::{closure
     1,153,485  ???:<rustc_middle::ty::subst::GenericArg as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with::<rustc_infer::infer::resolve::OpportunisticVarResolver>
     1,141,965  ???:<rustc_mir_build::thir::pattern::check_match::MatchVisitor>::check_irrefutable
    -1,132,923  ???:<rustc_middle::ty::Ty as rustc_type_ir::visit::TypeSuperVisitable<rustc_middle::ty::context::TyCtxt>>::super_visit_with::<rustc_privacy::DefIdVisitorSkeleton<rustc_privacy::SearchInterfaceForPrivateItemsVisitor>>
     1,124,766  ???:<rustc_middle::ty::sty::TraitRef as rustc_serialize::serialize::Encodable<rustc_metadata::rmeta::encoder::EncodeContext>>::encode
     1,118,371  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with::<rustc_infer::infer::resolve::OpportunisticVarResolver>
     1,111,056  ???:rustc_middle::ty::util::fold_list::<rustc_middle::ty::fold::RegionFolder, rustc_middle::ty::subst::GenericArg, <rustc_middle::ty::context::TyCtxt>::mk_substs>
    -1,109,967  ???:rustc_middle::ty::util::fold_list::<rustc_infer::infer::resolve::FullTypeResolver, rustc_middle::ty::subst::GenericArg, <&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with<rustc_infer::infer::resolve::FullTypeResolver>::{closure
     1,098,992  ???:<rustc_middle::ty::subst::EarlyBinder<rustc_middle::ty::sty::TraitRef>>::subst
     1,098,389  ???:rustc_middle::ty::util::fold_list::<rustc_infer::infer::resolve::FullTypeResolver, rustc_middle::ty::subst::GenericArg, <rustc_middle::ty::context::TyCtxt>::mk_substs>
    -1,092,685  ???:rustc_trait_selection::traits::normalize_param_env_or_error
    -1,086,806  ???:core::ptr::drop_in_place::<rustc_middle::traits::ObligationCauseCode>
    -1,071,086  ???:<rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>>::fill_item::<<rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>>::identity_for_item::{closure
    -1,070,514  ???:<rustc_middle::ty::sty::Region as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::fold_with::<rustc_middle::ty::fold::RegionFolder>
     1,061,815  ???:<&mut rustc_hir_analysis::check::wfcheck::check_where_clauses::{closure
    -1,047,949  ???:<rustc_infer::infer::equate::Equate as rustc_middle::ty::relate::TypeRelation>::tys
    -1,038,918  ???:<alloc::vec::Vec<rustc_infer::traits::Obligation<rustc_middle::ty::Predicate>> as alloc::vec::spec_extend::SpecExtend<rustc_infer::traits::Obligation<rustc_middle::ty::Predicate>, core::iter::adapters::filter::Filter<alloc::vec::into_iter::IntoIter<rustc_infer::traits::Obligation<rustc_middle::ty::Predicate>>, <rustc_infer::traits::util::Elaborator>::extend_deduped<alloc::vec::Vec<rustc_infer::traits::Obligation<rustc_middle::ty::Predicate>>>::{closure
    -1,036,249  ???:<rustc_infer::infer::outlives::verify::VerifyBoundCx>::alias_bound
    -1,031,811  ./string/../sysdeps/x86_64/multiarch/memmove-vec-unaligned-erms.S:__memcpy_sse2_unaligned_erms
     1,031,495  ???:<&mut rustc_hir_analysis::check::wfcheck::check_fn_or_method::{closure
       974,350  ???:<rustc_middle::ty::sty::TraitRef as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with::<rustc_infer::infer::resolve::OpportunisticVarResolver>
       953,894  ???:<rustc_middle::ty::subst::SubstFolder as rustc_type_ir::fold::TypeFolder<rustc_middle::ty::context::TyCtxt>>::fold_ty
      -932,564  ???:<rustc_infer::infer::InferCtxt>::probe::<core::result::Result<rustc_middle::traits::select::EvaluationResult, rustc_middle::traits::select::OverflowError>, <rustc_trait_selection::traits::select::SelectionContext>::evaluation_probe<<rustc_trait_selection::traits::select::SelectionContext>::evaluate_candidate::{closure
       896,298  ???:<alloc::vec::Vec<rustc_middle::thir::ExprId> as alloc::vec::spec_from_iter::SpecFromIter<rustc_middle::thir::ExprId, core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::iter::sources::once::Once<&rustc_hir::hir::Expr>, core::slice::iter::Iter<rustc_hir::hir::Expr>>, <rustc_mir_build::thir::cx::Cx>::make_mirror_unadjusted::{closure
       893,680  ???:<rustc_middle::ty::flags::FlagComputation>::for_kind
       888,204  ???:malloc
       887,340  ???:<ena::unify::UnificationTable<ena::unify::backing_vec::InPlace<rustc_type_ir::TyVid, &mut alloc::vec::Vec<ena::unify::VarValue<rustc_type_ir::TyVid>>, &mut rustc_infer::infer::undo_log::InferCtxtUndoLogs>>>::uninlined_get_root_key
       886,279  ???:<rustc_privacy::DefIdVisitorSkeleton<rustc_privacy::FindMin<rustc_middle::ty::Visibility>> as rustc_type_ir::visit::TypeVisitor<rustc_middle::ty::context::TyCtxt>>::visit_ty
       884,644  ???:<rustc_trait_selection::traits::engine::ObligationCtxt>::normalize::<rustc_middle::ty::sty::FnSig>
       878,482  ???:rustc_mir_dataflow::move_paths::builder::gather_moves
       877,364  ???:<rustc_infer::infer::InferCtxt>::probe::<(), <rustc_trait_selection::traits::select::SelectionContext>::assemble_candidates_from_object_ty::{closure
       873,833  ???:<rustc_middle::ty::fold::RegionFolder as rustc_type_ir::fold::FallibleTypeFolder<rustc_middle::ty::context::TyCtxt>>::try_fold_region
       866,348  ???:rustc_middle::ty::util::fold_list::<rustc_infer::infer::resolve::OpportunisticVarResolver, rustc_middle::ty::subst::GenericArg, <rustc_middle::ty::context::TyCtxt>::mk_substs>
       861,119  ???:<rustc_mir_dataflow::framework::engine::Engine<rustc_borrowck::dataflow::Borrows>>::iterate_to_fixpoint
      -861,089  ???:<rustc_middle::ty::sty::FnSig as rustc_trait_selection::traits::query::type_op::normalize::Normalizable>::type_op_method
       848,917  ???:<rustc_middle::infer::canonical::QueryRegionConstraints as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with::<rustc_infer::infer::canonical::canonicalizer::Canonicalizer>
      -848,756  ???:<rustc_infer::infer::outlives::obligations::TypeOutlives<&rustc_infer::infer::InferCtxt>>::components_must_outlive
      -844,184  ???:rustc_trait_selection::traits::specialize::translate_substs
       843,988  ???:<rustc_middle::ty::sty::Region as rustc_middle::ty::relate::Relate>::relate::<rustc_infer::infer::sub::Sub>
       833,458  ???:rustc_traits::type_op::type_op_ascribe_user_type_with_span
       830,842  ???:<rustc_infer::infer::InferCtxt>::instantiate_binder_with_fresh_vars::<rustc_middle::ty::sty::FnSig>
      -827,622  ???:<rustc_mir_build::thir::pattern::check_match::MatchVisitor as rustc_hir::intravisit::Visitor>::visit_param
       826,011  ???:<rustc_infer::infer::outlives::env::OutlivesEnvironment>::with_bounds::<core::iter::adapters::flatten::Flatten<core::iter::adapters::map::Map<indexmap::set::IntoIter<rustc_middle::ty::Ty>, <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::outlives_bounds::InferCtxtExt>::implied_bounds_tys::{closure
       820,974  ???:<rustc_middle::infer::canonical::Canonical<rustc_middle::ty::ParamEnvAnd<rustc_middle::traits::query::type_op::Normalize<rustc_middle::ty::Predicate>>> as rustc_infer::infer::canonical::substitute::CanonicalExt<rustc_middle::ty::ParamEnvAnd<rustc_middle::traits::query::type_op::Normalize<rustc_middle::ty::Predicate>>>>::substitute
      -814,281  ???:rustc_infer::infer::canonical::substitute::substitute_value::<rustc_middle::ty::ParamEnvAnd<rustc_middle::traits::query::type_op::Normalize<rustc_middle::ty::Predicate>>>
       808,938  ???:<rustc_infer::infer::resolve::OpportunisticVarResolver as rustc_type_ir::fold::FallibleTypeFolder<rustc_middle::ty::context::TyCtxt>>::try_fold_binder::<rustc_middle::ty::TraitPredicate>
       804,768  ???:<alloc::boxed::Box<rustc_middle::traits::ImplDerivedObligationCause> as rustc_type_ir::visit::TypeVisitable<rustc_middle::ty::context::TyCtxt>>::visit_with::<rustc_middle::ty::visit::HasEscapingVarsVisitor>
      -774,613  ???:<rustc_middle::ty::context::TyCtxt>::def_kind::<rustc_span::def_id::LocalDefId>
       763,260  ???:<rustc_hir_analysis::collect::ItemCtxt as rustc_hir_analysis::astconv::AstConv>::projected_ty_from_poly_trait_ref
       748,234  ???:core::iter::adapters::try_process::<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<rustc_middle::ty::Predicate>, <alloc::vec::Vec<rustc_middle::ty::Predicate> as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with<rustc_trait_selection::traits::project::AssocTypeNormalizer>::{closure
       731,392  ???:<rustc_middle::ty::subst::SubstFolder as rustc_type_ir::fold::FallibleTypeFolder<rustc_middle::ty::context::TyCtxt>>::try_fold_binder::<rustc_middle::ty::sty::FnSig>
       694,572  ???:rustc_middle::ty::util::fold_list::<rustc_middle::ty::subst::SubstFolder, rustc_middle::ty::Predicate, <rustc_middle::ty::context::TyCtxt>::intern_predicates>
       692,432  ???:<rustc_middle::ty::subst::GenericArg as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with::<rustc_middle::ty::erase_regions::RegionEraserVisitor>
      -690,629  ???:<rustc_middle::ty::context::TyCtxt>::mk_substs
       679,383  ???:<alloc::boxed::Box<rustc_middle::traits::ImplDerivedObligationCause> as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with::<rustc_infer::infer::resolve::OpportunisticVarResolver>
      -676,375  ???:<rustc_middle::ty::subst::GenericArg as rustc_type_ir::CollectAndApply<rustc_middle::ty::subst::GenericArg, &rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>>>::collect_and_apply::<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_middle::infer::canonical::CanonicalVarInfo>>>, <rustc_infer::infer::InferCtxt>::query_response_substitution_guess<rustc_middle::ty::Ty>::{closure
      -676,167  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::Ty> as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with::<rustc_middle::ty::fold::BoundVarReplacer<rustc_middle::ty::fold::FnMutDelegate>>
       673,706  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::fold_with::<rustc_infer::infer::resolve::OpportunisticVarResolver>
       668,629  ???:<rustc_middle::ty::context::TyCtxt>::mk_substs_from_iter::<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_middle::infer::canonical::CanonicalVarInfo>>>, <rustc_infer::infer::InferCtxt>::query_response_substitution_guess<rustc_middle::ty::Ty>::{closure
       663,126  ???:<core::option::Option<alloc::rc::Rc<rustc_middle::traits::ObligationCauseCode>> as rustc_type_ir::visit::TypeVisitable<rustc_middle::ty::context::TyCtxt>>::visit_with::<rustc_middle::ty::visit::HasTypeFlagsVisitor>
       659,039  ???:core::ptr::drop_in_place::<[rustc_infer::infer::region_constraints::VerifyBound]>
       647,438  ???:_rjem_je_tcache_bin_flush_small
       637,854  ???:<rustc_middle::ty::context::TyCtxt>::opt_associated_item
       637,419  ???:<rustc_mir_dataflow::framework::engine::Engine<rustc_mir_dataflow::impls::MaybeUninitializedPlaces>>::iterate_to_fixpoint
      -626,540  ???:<rustc_infer::infer::InferCtxt>::instantiate_binder_with_placeholders::<rustc_middle::ty::TraitPredicate>
      -619,118  ???:rustc_middle::ty::util::fold_list::<rustc_infer::infer::resolve::OpportunisticVarResolver, rustc_middle::ty::subst::GenericArg, <&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with<rustc_infer::infer::resolve::OpportunisticVarResolver>::{closure
      -603,076  ???:<rustc_middle::ty::Ty as rustc_type_ir::visit::TypeSuperVisitable<rustc_middle::ty::context::TyCtxt>>::super_visit_with::<<rustc_middle::ty::context::TyCtxt>::any_free_region_meets::RegionVisitor<<rustc_middle::ty::context::TyCtxt>::for_each_free_region<rustc_middle::ty::Ty, <rustc_borrowck::constraint_generation::ConstraintGeneration>::add_regular_live_constraint<rustc_middle::ty::Ty>::{closure
      -598,590  ???:<hashbrown::raw::RawTable<(rustc_infer::traits::project::ProjectionCacheKey, rustc_infer::traits::project::ProjectionCacheEntry)> as core::ops::drop::Drop>::drop
      -593,218  ???:<hashbrown::raw::RawTable<(rustc_span::def_id::DefId, (core::option::Option<rustc_middle::middle::stability::DeprecationEntry>, rustc_query_system::dep_graph::graph::DepNodeIndex))>>::reserve_rehash::<hashbrown::map::make_hasher<rustc_span::def_id::DefId, rustc_span::def_id::DefId, (core::option::Option<rustc_middle::middle::stability::DeprecationEntry>, rustc_query_system::dep_graph::graph::DepNodeIndex), core::hash::BuildHasherDefault<rustc_hash::FxHasher>>::{closure
       593,218  ???:<hashbrown::raw::RawTable<(rustc_span::def_id::DefId, (core::option::Option<rustc_middle::ty::subst::EarlyBinder<rustc_middle::ty::sty::TraitRef>>, rustc_query_system::dep_graph::graph::DepNodeIndex))>>::reserve_rehash::<hashbrown::map::make_hasher<rustc_span::def_id::DefId, rustc_span::def_id::DefId, (core::option::Option<rustc_middle::ty::subst::EarlyBinder<rustc_middle::ty::sty::TraitRef>>, rustc_query_system::dep_graph::graph::DepNodeIndex), core::hash::BuildHasherDefault<rustc_hash::FxHasher>>::{closure
       591,620  ???:<<rustc_middle::ty::context::TyCtxt>::any_free_region_meets::RegionVisitor<<rustc_middle::ty::context::TyCtxt>::for_each_free_region<rustc_middle::ty::Ty, <rustc_borrowck::constraint_generation::ConstraintGeneration>::add_regular_live_constraint<rustc_middle::ty::Ty>::{closure
       591,454  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::Ty> as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with::<rustc_trait_selection::traits::project::AssocTypeNormalizer>
       590,451  ???:<rustc_middle::ty::context::TyCtxt>::mk_predicates
       588,696  ???:<rustc_middle::mir::syntax::Place>::ty_from::<rustc_middle::mir::Body>
       588,476  ???:<rustc_infer::infer::at::At as rustc_trait_selection::traits::query::normalize::QueryNormalizeExt>::query_normalize::<rustc_middle::ty::sty::FnSig>
       587,786  ???:<rustc_middle::ty::sty::Region as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with::<rustc_infer::infer::resolve::FullTypeResolver>
      -587,322  ???:<rustc_middle::ty::Ty as rustc_type_ir::fold::TypeSuperFoldable<rustc_middle::ty::context::TyCtxt>>::super_fold_with::<rustc_middle::ty::fold::RegionFolder>
      -576,225  ???:rustc_middle::ty::util::fold_list::<rustc_middle::ty::fold::BoundVarReplacer<rustc_middle::ty::fold::FnMutDelegate>, rustc_middle::ty::subst::GenericArg, <&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with<rustc_middle::ty::fold::BoundVarReplacer<rustc_middle::ty::fold::FnMutDelegate>>::{closure
      -575,487  ???:<rustc_middle::ty::Ty as rustc_type_ir::visit::TypeSuperVisitable<rustc_middle::ty::context::TyCtxt>>::super_visit_with::<rustc_privacy::DefIdVisitorSkeleton<rustc_privacy::FindMin<rustc_middle::ty::Visibility>>>
       573,953  ???:rustc_passes::naked_functions::check_mod_naked_functions
       567,148  ???:<rustc_metadata::rmeta::encoder::EncodeContext as rustc_serialize::serialize::Encoder>::emit_enum_variant::<<rustc_type_ir::sty::TyKind<rustc_middle::ty::context::TyCtxt> as rustc_serialize::serialize::Encodable<rustc_metadata::rmeta::encoder::EncodeContext>>::encode::{closure
       557,906  ???:<rustc_passes::check_attr::CheckAttrVisitor as rustc_hir::intravisit::Visitor>::visit_expr
      -557,356  ???:rustc_mir_dataflow::drop_flag_effects::on_all_children_bits::on_all_children_bits::<rustc_mir_dataflow::drop_flag_effects::drop_flag_effects_for_location<<rustc_mir_dataflow::impls::MaybeInitializedPlaces as rustc_mir_dataflow::framework::GenKillAnalysis>::statement_effect<rustc_index::bit_set::ChunkedBitSet<rustc_mir_dataflow::move_paths::MovePathIndex>>::{closure
      -527,343  ???:<rustc_trait_selection::traits::wf::WfPredicates>::compute
      -526,940  ???:<ena::unify::UnificationTable<ena::unify::backing_vec::InPlace<rustc_type_ir::TyVid, &mut alloc::vec::Vec<ena::unify::VarValue<rustc_type_ir::TyVid>>, &mut rustc_infer::infer::undo_log::InferCtxtUndoLogs>>>::union::<rustc_type_ir::TyVid, rustc_type_ir::TyVid>
       499,156  ???:<alloc::boxed::Box<rustc_middle::traits::ImplDerivedObligationCause> as rustc_type_ir::visit::TypeVisitable<rustc_middle::ty::context::TyCtxt>>::visit_with::<rustc_middle::ty::visit::HasTypeFlagsVisitor>
       494,515  ???:rustc_passes::lang_items::get_lang_items
       494,144  ???:<rustc_infer::infer::ShallowResolver>::fold_infer_ty
      -484,694  ???:rustc_trait_selection::traits::do_normalize_predicates
       476,349  ???:<alloc::rc::Rc<rustc_middle::traits::ObligationCauseCode> as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with::<rustc_infer::infer::resolve::OpportunisticVarResolver>
       475,931  ???:<rustc_middle::mir::interpret::allocation::Allocation as rustc_data_structures::intern::Internable<rustc_middle::ty::context::TyCtxt>>::intern
       472,340  library/core/src/alloc/layout.rs:alloc::rc::rcbox_layout_for_value_layout
       470,910  ???:<rustc_middle::ty::sty::Binder<rustc_middle::ty::PredicateKind>>::dummy
      -469,860  ???:<rustc_middle::ty::context::TyCtxt>::mk_const_alloc
       465,338  ???:<alloc::rc::Rc<rustc_middle::traits::ObligationCauseCode> as rustc_type_ir::visit::TypeVisitable<rustc_middle::ty::context::TyCtxt>>::visit_with::<rustc_middle::ty::visit::HasTypeFlagsVisitor>
       458,563  ???:<rustc_middle::ty::sty::Region as rustc_middle::ty::relate::Relate>::relate::<rustc_infer::infer::equate::Equate>
      -456,504  ???:<rustc_middle::ty::Ty as rustc_type_ir::visit::TypeVisitable<rustc_middle::ty::context::TyCtxt>>::visit_with::<rustc_privacy::DefIdVisitorSkeleton<rustc_privacy::FindMin<rustc_middle::ty::Visibility>>>
      -455,715  ???:<alloc::vec::Vec<rustc_middle::infer::MemberConstraint> as rustc_type_ir::fold::TypeFoldable<rustc_middle::ty::context::TyCtxt>>::try_fold_with::<rustc_infer::infer::canonical::canonicalizer::Canonicalizer>
      -455,532  ???:<dyn rustc_hir_analysis::astconv::AstConv>::associated_path_to_ty::{closure
      -453,487  ???:rustc_traits::type_op::type_op_normalize::<rustc_middle::ty::sty::FnSig>
       452,489  ???:<core::option::Option<alloc::rc::Rc<rustc_middle::traits::ObligationCauseCode>> as rustc_type_ir::visit::TypeVisitable<rustc_middle::ty::context::TyCtxt>>::visit_with::<rustc_middle::ty::visit::HasEscapingVarsVisitor>
      -448,513  ???:core::ptr::drop_in_place::<rustc_infer::infer::region_constraints::VerifyBound>
       448,471  ???:rustc_traits::implied_outlives_bounds::compute_implied_outlives_bounds
      -448,459  ???:<rustc_middle::ty::sty::Binder<rustc_middle::ty::sty::FnSig> as rustc_serialize::serialize::Encodable<rustc_metadata::rmeta::encoder::EncodeContext>>::encode
       442,184  ???:<rustc_trait_selection::traits::select::SelectionContext>::match_projection_obligation_against_definition_bounds

--------------------------------------------------------------------------------
The following files chosen for auto-annotation could not be found:
--------------------------------------------------------------------------------
  ./string/../sysdeps/x86_64/multiarch/memmove-vec-unaligned-erms.S
  library/core/src/alloc/layout.rs
