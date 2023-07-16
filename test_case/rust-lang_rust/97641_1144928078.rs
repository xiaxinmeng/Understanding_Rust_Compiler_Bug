
 1,834,531  ???:<rustc_data_structures::obligation_forest::ObligationForest<rustc_trait_selection::traits::fulfill::PendingPredicateObligation>>::process_obligations::<rustc_trait_selection::traits::fulfill::FulfillProcessor, rustc_data_structures::obligation_forest::Outcome<rustc_trait_selection::traits::fulfill::PendingPredicateObligation, rustc_infer::traits::FulfillmentErrorCode>>
-1,237,662  ???:<rustc_infer::infer::combine::Generalizer as rustc_middle::ty::relate::TypeRelation>::relate_with_variance::<rustc_middle::ty::subst::GenericArg>
   960,255  ???:<rustc_middle::ty::subst::GenericArg as rustc_middle::ty::relate::Relate>::relate::<rustc_infer::infer::combine::Generalizer>
   880,869  ???:<rustc_infer::infer::lexical_region_resolve::LexicalResolver>::infer_variable_values
  -857,849  ???:rustc_infer::infer::lexical_region_resolve::resolve
   820,728  ???:<rustc_data_structures::obligation_forest::ObligationForest<rustc_trait_selection::traits::fulfill::PendingPredicateObligation>>::compress::<<rustc_data_structures::obligation_forest::ObligationForest<rustc_trait_selection::traits::fulfill::PendingPredicateObligation>>::process_obligations<rustc_trait_selection::traits::fulfill::FulfillProcessor, rustc_data_structures::obligation_forest::Outcome<rustc_trait_selection::traits::fulfill::PendingPredicateObligation, rustc_infer::traits::FulfillmentErrorCode>>::{closure
   662,607  ???:<rustc_data_structures::obligation_forest::ObligationForest<rustc_trait_selection::traits::fulfill::PendingPredicateObligation>>::apply_rewrites
  -564,995  ???:<rustc_infer::infer::equate::Equate as rustc_middle::ty::relate::TypeRelation>::relate_with_variance::<rustc_middle::ty::subst::GenericArg>
   551,065  ???:<rustc_middle::ty::subst::GenericArg as rustc_middle::ty::relate::Relate>::relate::<rustc_infer::infer::equate::Equate>
  -445,653  ???:<rustc_middle::ty::subst::GenericArg>::walk_shallow
   432,159  ???:<rustc_middle::ty::context::CtxtInterners>::intern_predicate
   404,411  ???:<rustc_data_structures::obligation_forest::ObligationForest<rustc_trait_selection::traits::fulfill::PendingPredicateObligation>>::uninlined_mark_dependents_as_waiting
   342,809  ???:<rustc_trait_selection::traits::fulfill::FulfillProcessor>::process_changed_obligations
  -327,517  ???:<rustc_infer::infer::combine::CombineFields>::instantiate
   272,601  ???:<rustc_infer::infer::equate::Equate as rustc_middle::ty::relate::TypeRelation>::tys
   264,798  ???:<rustc_infer::infer::InferCtxt>::nested_outlives_obligations
  -182,764  ???:rustc_infer::infer::outlives::components::compute_components_recursive
   164,839  ???:<rustc_data_structures::obligation_forest::ObligationForest<rustc_trait_selection::traits::fulfill::PendingPredicateObligation>>::find_cycles_from_node::<rustc_trait_selection::traits::fulfill::FulfillProcessor>
   131,944  ???:<rustc_infer::infer::InferCtxt>::resolve_regions_and_report_errors
   123,166  ???:free
  -112,131  ???:<rustc_middle::ty::context::TyCtxt>::intern_substs
  -105,280  ???:<alloc::vec::Vec<(rustc_middle::ty::sty::RegionVid, rustc_middle::ty::sty::RegionVid)>>::retain::<<rustc_infer::infer::lexical_region_resolve::LexicalResolver>::expansion::{closure
    99,428  ???:<hashbrown::raw::RawTable<(rustc_middle::ty::ParamEnvAnd<rustc_middle::ty::Predicate>, ())>>::reserve_rehash::<hashbrown::map::make_hasher<rustc_middle::ty::ParamEnvAnd<rustc_middle::ty::Predicate>, rustc_middle::ty::ParamEnvAnd<rustc_middle::ty::Predicate>, (), core::hash::BuildHasherDefault<rustc_hash::FxHasher>>::{closure
    95,105  ???:malloc
