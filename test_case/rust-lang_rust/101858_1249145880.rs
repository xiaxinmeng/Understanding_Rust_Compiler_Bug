
139,443,901  ???:<rustc_trait_selection::traits::fulfill::FulfillProcessor as rustc_data_structures::obligation_forest::ObligationProcessor>::process_obligation
 61,282,965  ???:<rustc_middle::ty::subst::SubstFolder as rustc_middle::ty::fold::TypeFolder>::fold_ty
-59,795,752  ???:<alloc::rc::Rc<rustc_middle::traits::ObligationCauseCode> as core::ops::drop::Drop>::drop
 49,456,068  ???:<rustc_middle::ty::subst::SubstFolder as rustc_middle::ty::fold::FallibleTypeFolder>::try_fold_ty
-47,967,367  ???:<rustc_middle::ty::Ty as rustc_middle::ty::fold::TypeSuperFoldable>::super_fold_with::<rustc_middle::ty::subst::SubstFolder>
 46,111,073  ???:<rustc_trait_selection::traits::select::SelectionContext>::evaluate_predicates_recursively::<alloc::vec::into_iter::IntoIter<rustc_infer::traits::Obligation<rustc_middle::ty::Predicate>>>
 44,568,113  ???:<rustc_infer::infer::combine::CombineFields>::instantiate
 33,973,909  ???:<rustc_trait_selection::traits::fulfill::FulfillmentContext as rustc_infer::traits::engine::TraitEngine>::register_predicate_obligation
-31,618,494  ???:<rustc_infer::infer::equate::Equate as rustc_middle::ty::relate::TypeRelation>::relate::<rustc_middle::ty::Ty>
 30,370,709  ???:<rustc_trait_selection::traits::wf::WfPredicates>::nominal_obligations_inner
 29,798,110  ???:rustc_trait_selection::traits::wf::predicate_obligations
 26,729,122  ???:<rustc_trait_selection::traits::select::SelectionContext>::assemble_candidates
 25,095,611  ???:<rustc_infer::infer::InferCtxt>::commit_if_ok::<rustc_infer::infer::InferOk<(alloc::vec::Vec<rustc_middle::ty::adjustment::Adjustment>, rustc_middle::ty::Ty)>, rustc_middle::ty::error::TypeError, <rustc_typeck::check::fn_ctxt::FnCtxt>::try_coerce::{closure
-24,951,977  ???:<rustc_trait_selection::traits::select::SelectionContext>::evaluate_predicate_recursively
-23,810,814  ???:<rustc_typeck::check::coercion::Coerce>::coerce
 23,372,325  ???:<rustc_middle::ty::ParamEnvAnd<rustc_middle::traits::query::type_op::ProvePredicate> as rustc_trait_selection::traits::query::type_op::TypeOp>::fully_perform
 20,236,839  ???:<alloc::vec::Vec<rustc_infer::traits::Obligation<rustc_middle::ty::Predicate>> as alloc::vec::spec_from_iter::SpecFromIter<rustc_infer::traits::Obligation<rustc_middle::ty::Predicate>, core::iter::adapters::filter::Filter<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<alloc::vec::into_iter::IntoIter<rustc_middle::ty::Predicate>, alloc::vec::into_iter::IntoIter<rustc_span::span_encoding::Span>>, core::iter::adapters::rev::Rev<alloc::vec::into_iter::IntoIter<rustc_span::def_id::DefId>>>, <rustc_trait_selection::traits::wf::WfPredicates>::nominal_obligations_inner::{closure
 17,899,560  ???:<rustc_trait_selection::traits::select::SelectionContext>::select
