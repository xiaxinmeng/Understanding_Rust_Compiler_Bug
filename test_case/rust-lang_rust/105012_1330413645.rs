
--------------------------------------------------------------------------------
Ir         
--------------------------------------------------------------------------------
57,155,936  PROGRAM TOTALS

--------------------------------------------------------------------------------
Ir           file:function
--------------------------------------------------------------------------------
 58,696,222  ???:<rustc_middle::ty::subst::GenericArg as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_infer::infer::resolve::OpportunisticVarResolver>
-49,549,818  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_infer::infer::resolve::OpportunisticVarResolver>
 33,374,397  ???:<rustc_middle::ty::Ty as rustc_middle::ty::fold::TypeSuperFoldable>::super_fold_with::<rustc_infer::infer::resolve::OpportunisticVarResolver>
-32,572,741  ???:rustc_middle::ty::util::fold_list::<rustc_infer::infer::freshen::TypeFreshener, rustc_middle::ty::Ty, <&rustc_middle::ty::list::List<rustc_middle::ty::Ty> as rustc_middle::ty::fold::TypeFoldable>::try_fold_with<rustc_infer::infer::freshen::TypeFreshener>::{closure
 30,287,775  ???:<rustc_middle::ty::Ty as rustc_middle::ty::fold::TypeSuperFoldable>::super_fold_with::<rustc_infer::infer::freshen::TypeFreshener>
 12,076,808  ???:<rustc_infer::infer::ShallowResolver as rustc_middle::ty::fold::TypeFolder>::fold_ty
  5,009,264  ???:<rustc_middle::ty::Ty as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_infer::infer::freshen::TypeFreshener>
  3,392,061  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_middle::ty::fold::RegionFolder>
 -2,596,008  ???:<rustc_middle::ty::context::TyCtxt>::intern_substs
 -2,105,899  ???:<rustc_middle::ty::Ty as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_middle::ty::fold::RegionFolder>
