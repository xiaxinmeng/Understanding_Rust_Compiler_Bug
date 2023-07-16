
274,500,092  ???:<rustc_infer::infer::resolve::OpportunisticVarResolver as rustc_middle::ty::fold::TypeFolder>::fold_ty
-77,218,487  ???:<rustc_middle::ty::Ty as rustc_middle::ty::fold::TypeSuperFoldable>::super_fold_with::<rustc_infer::infer::resolve::OpportunisticVarResolver>
-64,226,940  ???:<rustc_trait_selection::traits::fulfill::FulfillmentContext as rustc_infer::traits::engine::TraitEngine>::register_predicate_obligation
-32,954,717  ???:<rustc_typeck::check::fn_ctxt::FnCtxt>::instantiate_value_path
 31,759,842  ???:<rustc_typeck::check::fn_ctxt::FnCtxt>::check_expr_path
 17,598,127  ???:<alloc::vec::Vec<rustc_middle::traits::query::OutlivesBound> as alloc::vec::spec_extend::SpecExtend<rustc_middle::traits::query::OutlivesBound, core::iter::adapters::flatten::FlatMap<alloc::vec::into_iter::IntoIter<rustc_infer::traits::Obligation<rustc_middle::ty::Predicate>>, alloc::vec::Vec<rustc_middle::traits::query::OutlivesBound>, rustc_traits::implied_outlives_bounds::compute_implied_outlives_bounds::{closure
-15,764,188  ???:<&mut rustc_traits::implied_outlives_bounds::compute_implied_outlives_bounds::{closure
-10,797,979  ???:<rustc_infer::infer::InferCtxt>::probe::<core::result::Result<rustc_middle::traits::select::EvaluationResult, rustc_middle::traits::select::OverflowError>, rustc_trait_selection::traits::relationships::update<rustc_trait_selection::traits::fulfill::FulfillmentContext>::{closure
 10,161,726  ???:<rustc_infer::infer::InferCtxt as rustc_trait_selection::traits::query::evaluate_obligation::InferCtxtExt>::predicate_must_hold_modulo_regions
  9,708,632  ???:<rustc_infer::infer::canonical::canonicalizer::Canonicalizer>::canonicalize::<rustc_middle::ty::ParamEnvAnd<rustc_middle::ty::Predicate>>
 -9,659,170  ???:rustc_trait_selection::traits::type_known_to_meet_bound_modulo_regions
  5,231,325  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_infer::infer::resolve::OpportunisticVarResolver>
 -5,197,845  ???:<rustc_trait_selection::traits::fulfill::FulfillmentContext as rustc_infer::traits::engine::TraitEngine>::register_bound
  3,774,517  ???:rustc_middle::ty::util::fold_list::<rustc_infer::infer::resolve::OpportunisticVarResolver, rustc_middle::ty::Ty, <&rustc_middle::ty::list::List<rustc_middle::ty::Ty> as rustc_middle::ty::fold::TypeFoldable>::try_fold_with<rustc_infer::infer::resolve::OpportunisticVarResolver>::{closure
  3,454,665  ???:<rustc_infer::infer::InferCtxt>::canonicalize_query_keep_static::<rustc_middle::ty::ParamEnvAnd<rustc_middle::traits::query::type_op::Normalize<rustc_middle::ty::sty::FnSig>>>
 -3,403,813  /build/glibc-sMfBJT/glibc-2.31/string/../sysdeps/x86_64/multiarch/memmove-vec-unaligned-erms.S:__memcpy_sse2_unaligned_erms
 -2,631,684  ???:<rustc_infer::infer::resolve::OpportunisticVarResolver as rustc_middle::ty::fold::FallibleTypeFolder>::try_fold_ty
  2,540,584  ???:<rustc_middle::ty::Ty as rustc_middle::ty::fold::TypeFoldable>::fold_with::<rustc_infer::infer::resolve::OpportunisticVarResolver>
 -2,506,502  ???:<rustc_middle::ty::subst::GenericArg as rustc_type_ir::InternIteratorElement<rustc_middle::ty::subst::GenericArg, &rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg>>>::intern_with::<core::iter::adapters::chain::Chain<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_middle::ty::subst::GenericArg>>, core::iter::adapters::skip::Skip<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_middle::ty::subst::GenericArg>>>>, <rustc_middle::ty::context::TyCtxt>::mk_substs<core::iter::adapters::chain::Chain<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_middle::ty::subst::GenericArg>>, core::iter::adapters::skip::Skip<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_middle::ty::subst::GenericArg>>>>>::{closure
 -2,503,099  ???:rustc_traits::implied_outlives_bounds::compute_implied_outlives_bounds
 -2,471,662  ???:<rustc_middle::ty::walk::TypeWalker as core::iter::traits::iterator::Iterator>::next
  2,424,639  ???:<rustc_middle::ty::context::TyCtxt>::mk_substs::<core::iter::adapters::chain::Chain<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_middle::ty::subst::GenericArg>>, core::iter::adapters::skip::Skip<core::iter::adapters::copied::Copied<core::slice::iter::Iter<rustc_middle::ty::subst::GenericArg>>>>>
 -2,362,244  ???:<rustc_mir_build::thir::pattern::PatCtxt>::lower_variant_or_leaf
 -2,067,479  ???:<rustc_infer::infer::canonical::canonicalizer::Canonicalizer as rustc_middle::ty::fold::FallibleTypeFolder>::try_fold_predicate
  2,010,382  ???:<rustc_mir_build::thir::pattern::PatCtxt>::lower_pattern
 -1,834,405  ???:<rustc_borrowck::type_check::TypeChecker>::typeck_mir
 -1,768,568  ???:<rustc_privacy::DefIdVisitorSkeleton<rustc_privacy::TypePrivacyVisitor>>::visit_trait
 -1,475,131  ???:<rustc_infer::infer::ShallowResolver as rustc_middle::ty::fold::TypeFolder>::fold_ty
  1,423,064  ???:<rustc_privacy::TypePrivacyVisitor as rustc_hir::intravisit::Visitor>::visit_trait_ref
  1,241,238  ???:<rustc_trait_selection::traits::fulfill::FulfillProcessor>::process_changed_obligations
