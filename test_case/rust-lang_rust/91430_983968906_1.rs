
   4: <rustc_errors::diagnostic_builder::DiagnosticBuilder>::emit
   5: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::InferCtxtExt>::report_overflow_error::<rustc_middle::ty::Predicate>
   6: <rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::error_reporting::InferCtxtExt>::report_overflow_error_cycle
   7: <rustc_trait_selection::traits::fulfill::FulfillProcessor as rustc_data_structures::obligation_forest::ObligationProcessor>::process_backedge::<core::iter::adapters::map::Map<core::slice::iter::Iter<usize>, <rustc_data_structures::obligation_forest::ObligationForest<rustc_trait_selection::traits::fulfill::PendingPredicateObligation>>::find_cycles_from_node<rustc_trait_selection::traits::fulfill::FulfillProcessor>::{closure#1}>>
   8: <rustc_data_structures::obligation_forest::ObligationForest<rustc_trait_selection::traits::fulfill::PendingPredicateObligation>>::find_cycles_from_node::<rustc_trait_selection::traits::fulfill::FulfillProcessor>
   9: <rustc_data_structures::obligation_forest::ObligationForest<rustc_trait_selection::traits::fulfill::PendingPredicateObligation>>::find_cycles_from_node::<rustc_trait_selection::traits::fulfill::FulfillProcessor>
  10: <rustc_data_structures::obligation_forest::ObligationForest<rustc_trait_selection::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection::traits::fulfill::FulfillProcessor, rustc_data_structures::obligation_forest::Outcome<rustc_trait_selection::traits::fulfill::PendingPredicateObligation, rustc_infer::traits::FulfillmentErrorCode>>
  11: <rustc_trait_selection::traits::fulfill::FulfillmentContext as rustc_infer::traits::engine::TraitEngine>::select_where_possible
  12: <rustc_infer::infer::InferCtxtBuilder as rustc_trait_selection::infer::InferCtxtBuilderExt>::enter_canonical_trait_query::<rustc_middle::ty::ParamEnvAnd<rustc_middle::ty::sty::ProjectionTy>, rustc_middle::traits::query::NormalizationResult, rustc_traits::normalize_projection_ty::normalize_projection_ty::{closure#0}>
  13: rustc_traits::normalize_projection_ty::normalize_projection_ty
  14: rustc_query_system::query::plumbing::try_execute_query::<rustc_query_impl::plumbing::QueryCtxt, rustc_query_system::query::caches::DefaultCache<rustc_middle::infer::canonical::Canonical<rustc_middle::ty::ParamEnvAnd<rustc_middle::ty::sty::ProjectionTy>>, core::result::Result<&rustc_middle::infer::canonical::Canonical<rustc_middle::infer::canonical::QueryResponse<rustc_middle::traits::query::NormalizationResult>>, rustc_middle::traits::query::NoSolution>>>
  15: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::normalize_projection_ty, rustc_query_impl::plumbing::QueryCtxt>
  16: <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::normalize_projection_ty
  17: <rustc_trait_selection::traits::query::normalize::QueryNormalizer as rustc_middle::ty::fold::TypeFolder>::fold_ty
  18: <rustc_infer::infer::at::At as rustc_trait_selection::traits::query::normalize::AtExt>::normalize::<rustc_middle::ty::subst::GenericArg>
  19: <rustc_infer::infer::InferCtxtBuilder>::enter::<core::result::Result<rustc_middle::ty::subst::GenericArg, rustc_middle::traits::query::NoSolution>, rustc_traits::normalize_erasing_regions::try_normalize_after_erasing_regions<rustc_middle::ty::subst::GenericArg>::{closure#0}>
  20: <rustc_traits::normalize_erasing_regions::provide::{closure#2} as core::ops::function::FnOnce<(rustc_middle::ty::context::TyCtxt, rustc_middle::ty::ParamEnvAnd<rustc_middle::ty::subst::GenericArg>)>>::call_once
  21: rustc_query_system::query::plumbing::get_query::<rustc_query_impl::queries::try_normalize_generic_arg_after_erasing_regions, rustc_query_impl::plumbing::QueryCtxt>
  22: <rustc_middle::ty::normalize_erasing_regions::TryNormalizeAfterErasingRegionsFolder>::try_normalize_generic_arg_after_erasing_regions
  23: <rustc_middle::ty::normalize_erasing_regions::TryNormalizeAfterErasingRegionsFolder as rustc_middle::ty::fold::TypeFolder>::fold_ty
  24: <&rustc_middle::ty::TyS as rustc_middle::ty::fold::TypeFoldable>::fold_with::<rustc_middle::ty::normalize_erasing_regions::TryNormalizeAfterErasingRegionsFolder>
             at /rustc/f04a2f4b8e89eac1119061ea2055d33c97e618b4/compiler/rustc_middle/src/ty/structural_impls.rs:913:9
  25: <rustc_middle::ty::context::TyCtxt>::try_normalize_erasing_regions::<&rustc_middle::ty::TyS>
             at /rustc/f04a2f4b8e89eac1119061ea2055d33c97e618b4/compiler/rustc_middle/src/ty/normalize_erasing_regions.rs:93:13
  26: rustdoc::clean::normalize
