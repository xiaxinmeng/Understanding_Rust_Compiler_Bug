
--------------------------------------------------------------------------------
Ir
--------------------------------------------------------------------------------
488,637,125  PROGRAM TOTALS

--------------------------------------------------------------------------------
Ir           file:function
--------------------------------------------------------------------------------
115,583,666  ???:<rustc_middle::ty::fold::HasTypeFlagsVisitor as rustc_middle::ty::fold::TypeVisitor>::visit_ty
 72,854,689  ???:<rustc_middle::ty::subst::SubstFolder as rustc_middle::ty::fold::TypeFolder>::fold_ty
 47,512,130  ???:rustc_infer::infer::at::At::sub_exp
 45,945,600  ???:rustc_infer::infer::region_constraints::RegionConstraintCollector::rollback_to
 43,219,576  ???:rustc_infer::infer::region_constraints::RegionConstraintCollector::start_snapshot
 33,203,704  ???:rustc_infer::infer::InferCtxt::commit_if_ok
 30,939,291  ???:rustc_infer::infer::InferCtxt::start_snapshot
 29,405,184  ???:rustc_infer::infer::InferCtxt::rollback_to
 28,227,544  ???:rustc_infer::infer::InferCtxt::probe
-22,882,155  ???:<core::iter::adapters::Copied<I> as core::iter::traits::iterator::Iterator>::try_fold
-17,317,321  ???:rustc_trait_selection::traits::select::SelectionContext::match_projection
 14,711,112  ???:rustc_infer::infer::undo_log::<impl rustc_infer::infer::InferCtxtInner>::rollback_to
 11,867,495  ???:rustc_infer::infer::higher_ranked::<impl rustc_infer::infer::combine::CombineFields>::higher_ranked_sub
 11,248,672  ???:<rustc_infer::infer::equate::Equate as rustc_middle::ty::relate::TypeRelation>::tys
  6,987,960  ???:<rustc_middle::ty::fold::HasTypeFlagsVisitor as rustc_middle::ty::structural_impls::PredicateVisitor>::visit_predicate
  6,353,522  ???:rustc_middle::ty::context::TyCtxt::features
  6,330,920  ???:rustc_infer::infer::higher_ranked::<impl rustc_infer::infer::InferCtxt>::replace_bound_vars_with_placeholders
 -5,434,025  /rustc/4745cbe83e0b3299bfe7f7f305b975c3c09f92db//library/core/src/str/mod.rs:core::str::from_utf8
