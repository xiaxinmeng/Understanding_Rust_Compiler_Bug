
  300,939,473  ???:<rustc_middle::ty::fold::RegionFolder as rustc_middle::ty::fold::FallibleTypeFolder>::try_fold_ty
 -257,897,264  ???:<rustc_middle::ty::Ty as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_middle::ty::fold::RegionFolder>
   67,497,361  ???:rustc_middle::ty::util::fold_list::<rustc_middle::ty::fold::RegionFolder, rustc_middle::ty::subst::GenericArg, <&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_middle::ty::fold::TypeFoldable>::try_fold_with<rustc_middle::ty::fold::RegionFolder>::{closure#0}>
  -55,028,215  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_middle::ty::fold::RegionFolder>
   30,150,640  ???:<rustc_middle::ty::Ty as rustc_middle::ty::fold::TypeFoldable>::fold_with::<rustc_middle::ty::fold::RegionFolder>
  -25,360,097  ???:<rustc_middle::ty::Ty as rustc_middle::ty::fold::TypeSuperFoldable>::super_fold_with::<rustc_middle::ty::fold::RegionFolder>
   -7,820,135  ???:<rustc_middle::ty::subst::GenericArg as rustc_middle::ty::visit::TypeVisitable>::visit_with::<rustc_privacy::DefIdVisitorSkeleton<rustc_privacy::TypePrivacyVisitor>>
   -7,294,284  ???:<rustc_infer::infer::resolve::OpportunisticVarResolver as rustc_middle::ty::fold::TypeFolder>::fold_ty
    4,230,287  ???:<rustc_middle::ty::Ty as rustc_middle::ty::fold::TypeSuperFoldable>::super_fold_with::<rustc_infer::infer::freshen::TypeFreshener>
    3,402,246  ???:<rustc_middle::ty::Ty as rustc_middle::ty::fold::TypeSuperFoldable>::super_fold_with::<rustc_infer::infer::resolve::OpportunisticVarResolver>
    2,359,900  ???:<rustc_infer::infer::resolve::OpportunisticVarResolver as rustc_middle::ty::fold::FallibleTypeFolder>::try_fold_ty
   -1,827,113  ???:<rustc_middle::ty::Ty as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_infer::infer::freshen::TypeFreshener>
   -1,630,338  ???:<rustc_infer::infer::freshen::TypeFreshener as rustc_middle::ty::fold::FallibleTypeFolder>::try_fold_ty
    1,186,867  ???:<&rustc_middle::ty::list::List<rustc_middle::ty::subst::GenericArg> as rustc_middle::ty::fold::TypeFoldable>::try_fold_with::<rustc_infer::infer::resolve::OpportunisticVarResolver>
     -783,182  ???:<rustc_infer::infer::freshen::TypeFreshener as rustc_middle::ty::fold::TypeFolder>::fold_ty
     -588,848  ???:<rustc_infer::infer::resolve::FullTypeResolver as rustc_middle::ty::fold::FallibleTypeFolder>::try_fold_ty
      510,288  ???:<rustc_middle::ty::Ty as rustc_middle::ty::fold::TypeSuperFoldable>::try_super_fold_with::<rustc_infer::infer::resolve::FullTypeResolver>
