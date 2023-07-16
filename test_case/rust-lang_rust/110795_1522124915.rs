
-27,475,900  ???:<rustc_trait_selection::traits::select::SelectionContext>::candidate_from_obligation_no_cache
 23,589,873  ???:<rustc_middle::ty::context::TyCtxt>::for_each_relevant_impl_treating_projections::<<rustc_trait_selection::traits::select::SelectionContext>::assemble_candidates_from_impls::{closure#0}>
 18,336,865  ???:<rustc_middle::ty::fast_reject::DeepRejectCtxt>::types_may_unify
 14,753,570  ???:<rustc_middle::ty::context::TyCtxt>::impls_are_allowed_to_overlap
 11,340,173  ???:rustc_mir_transform::simplify::simplify_cfg
-11,314,840  ???:<rustc_mir_transform::simplify::SimplifyCfg as rustc_middle::mir::MirPass>::run_pass
  9,753,599  ???:<rustc_trait_selection::traits::select::SelectionContext>::select
  6,678,009  ???:<rustc_infer::infer::at::At>::eq::<rustc_middle::ty::sty::TraitRef>
  5,764,176  ???:<rustc_middle::traits::specialization_graph::Children as rustc_trait_selection::traits::specialize::specialization_graph::ChildrenExt>::insert
 -4,425,298  ???:<rustc_trait_selection::traits::select::SelectionContext>::match_impl
 -3,586,951  ???:rustc_trait_selection::traits::project::normalize_projection_type
  3,264,027  ???:rustc_trait_selection::traits::project::opt_normalize_projection_type
 -3,227,039  ???:<core::iter::sources::from_generator::FromGenerator<<rustc_metadata::creader::CrateMetadataRef>::get_module_children::{closure#0}> as core::iter::traits::iterator::Iterator>::next
  2,913,321  ???:<rustc_middle::metadata::ModChild as rustc_serialize::serialize::Decodable<rustc_metadata::rmeta::decoder::DecodeContext>>::decode
  2,714,082  ???:<core::iter::adapters::GenericShunt<core::iter::adapters::flatten::FlatMap<core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<rustc_middle::traits::select::SelectionCandidate>, <rustc_trait_selection::traits::select::SelectionContext>::candidate_from_obligation_no_cache::{closure#0}>, core::option::Option<core::result::Result<rustc_trait_selection::traits::select::EvaluatedCandidate, rustc_middle::traits::SelectionError>>, <core::result::Result<core::option::Option<rustc_trait_selection::traits::select::EvaluatedCandidate>, rustc_middle::traits::SelectionError>>::transpose>, core::result::Result<core::convert::Infallible, rustc_middle::traits::SelectionError>> as core::iter::traits::iterator::Iterator>::next
 -2,140,674  ???:<rustc_trait_selection::traits::query::type_op::implied_outlives_bounds::ImpliedOutlivesBounds as rustc_trait_selection::traits::query::type_op::QueryTypeOp>::fully_perform_into
  1,822,103  ???:<rustc_trait_selection::traits::select::SelectionContext>::assemble_candidates
 -1,596,256  ???:<rustc_infer::infer::freshen::TypeFreshener as rustc_type_ir::fold::FallibleTypeFolder<rustc_middle::ty::context::TyCtxt>>::try_fold_ty
 -1,588,995  ???:<rustc_middle::ty::context::TyCtxt>::typeck_body
 -1,487,660  ???:<rustc_trait_selection::traits::fulfill::FulfillmentContext as rustc_infer::traits::engine::TraitEngine>::register_predicate_obligation
  1,428,508  ???:<rustc_middle::hir::map::Map>::body_owner_def_id
  1,387,303  ???:<rustc_trait_selection::traits::engine::ObligationCtxt>::register_obligation
  1,359,204  ???:<rustc_middle::ty::ParamEnvAnd<rustc_trait_selection::traits::query::type_op::implied_outlives_bounds::ImpliedOutlivesBounds> as rustc_trait_selection::traits::query::type_op::TypeOp>::fully_perform
  1,273,349  ???:<rustc_infer::infer::InferCtxt>::instantiate_nll_query_response_and_region_obligations::<alloc::vec::Vec<rustc_middle::traits::query::OutlivesBound>>
  1,076,454  ???:<rustc_middle::ty::Ty as rustc_type_ir::fold::TypeSuperFoldable<rustc_middle::ty::context::TyCtxt>>::super_fold_with::<rustc_infer::infer::freshen::TypeFreshener>
 -1,055,849  ???:<rustc_trait_selection::traits::engine::ObligationCtxt>::normalize::<rustc_middle::ty::Predicate>
  1,013,162  ???:<rustc_infer::infer::at::At as rustc_trait_selection::traits::project::NormalizeExt>::normalize::<rustc_middle::ty::Predicate>
    776,671  ???:<rustc_middle::ty::Ty as rustc_type_ir::fold::TypeSuperFoldable<rustc_middle::ty::context::TyCtxt>>::super_fold_with::<rustc_middle::ty::subst::SubstFolder>
   -774,633  ???:<rustc_middle::ty::Ty as rustc_serialize::serialize::Decodable<rustc_metadata::rmeta::decoder::DecodeContext>>::decode
    764,497  ???:rustc_query_system::query::plumbing::try_get_cached::<rustc_middle::ty::context::TyCtxt, rustc_query_system::query::caches::DefaultCache<rustc_span::def_id::DefId, rustc_middle::query::erase::Erased<[u8; 8]>>>
   -711,779  ???:<rustc_trait_selection::traits::project::AssocTypeNormalizer as rustc_type_ir::fold::FallibleTypeFolder<rustc_middle::ty::context::TyCtxt>>::try_fold_predicate
    711,559  ???:<rustc_middle::ty::Predicate as rustc_type_ir::fold::TypeSuperFoldable<rustc_middle::ty::context::TyCtxt>>::super_fold_with::<rustc_trait_selection::traits::project::AssocTypeNormalizer>
    620,599  ???:<core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<rustc_middle::mir::LocalDecl>>, <rustc_index::slice::IndexSlice<rustc_middle::mir::Local, rustc_middle::mir::LocalDecl>>::iter_enumerated::{closure#0}> as itertools::Itertools>::partition_map::<alloc::vec::Vec<rustc_middle::mir::Local>, alloc::vec::Vec<rustc_middle::mir::Local>, rustc_borrowck::type_check::liveness::compute_relevant_live_locals::{closure#0}, rustc_middle::mir::Local, rustc_middle::mir::Local>
