
 57,039,437  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::Ty> as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_middle::ty::fold::RegionFolder>
 18,020,280  ???:<rustc_middle::ty::Ty as rustc_middle::ty::fold::TypeSuperFoldable>::super_fold_with::<rustc_middle::ty::fold::RegionFolder>
-17,691,750  ???:<rustc_middle::ty::fold::RegionFolder as rustc_middle::ty::fold::FallibleTypeFolder>::try_fold_binder::<&rustc_middle::ty::list::List<rustc_middle::ty::Ty>>
 17,691,750  ???:<rustc_middle::ty::sty::Binder<&rustc_middle::ty::list::List<rustc_middle::ty::Ty>> as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_middle::ty::fold::RegionFolder>
 15,780,887  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_middle::ty::fold::RegionFolder>
  7,517,816  ???:<rustc_middle::ty::fold::RegionFolder as rustc_middle::ty::fold::FallibleTypeFolder>::try_fold_ty
  2,328,794  ???:<rustc_type_ir::sty::TyKind<rustc_middle::ty::context::TyCtxt> as core::cmp::PartialEq>::eq
  2,136,544  ???:<rustc_infer::infer::InferCtxt>::commit_unconditionally::<rustc_middle::traits::ImplSourceUserDefinedData<rustc_infer::traits::Obligation<rustc_middle::ty::Predicate>>, <rustc_trait_selection::traits::select::SelectionContext>::confirm_impl_candidate::{closure
  2,124,505  ???:<rustc_infer::infer::resolve::OpportunisticVarResolver as rustc_middle::ty::fold::TypeFolder>::fold_ty
 -2,063,444  ???:<rustc_trait_selection::traits::select::SelectionContext>::vtable_impl
  2,030,069  ???:<rustc_middle::ty::context::CtxtInterners>::intern_ty
 -1,822,536  ???:<rustc_middle::ty::Ty as rustc_middle::ty::fold::TypeSuperFoldable>::super_fold_with::<rustc_infer::infer::resolve::OpportunisticVarResolver>
  1,252,075  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_infer::infer::resolve::OpportunisticVarResolver>
   -660,499  ???:<rustc_infer::infer::resolve::OpportunisticVarResolver as rustc_middle::ty::fold::FallibleTypeFolder>::try_fold_ty
