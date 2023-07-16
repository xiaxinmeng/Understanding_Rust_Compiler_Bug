
#0  0x00007ff809464ebd in <rustc_middle[ebf260dc149f6367]::traits::ObligationCauseCode as core[20e502c22ca6166f]::cmp::PartialEq>::eq ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#1  0x00007ff80946554a in <rustc_middle[ebf260dc149f6367]::traits::ImplDerivedObligationCause as core[20e502c22ca6166f]::cmp::PartialEq>::eq ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
---collapsed 61 identical stack frames---
#62 0x00007ff80946554a in <rustc_middle[ebf260dc149f6367]::traits::ImplDerivedObligationCause as core[20e502c22ca6166f]::cmp::PartialEq>::eq ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#63 0x00007ff809463629 in <hashbrown[a8280373705225ad]::map::HashMap<rustc_infer[f1d617581dd2c354]::traits::Obligation<rustc_middle[ebf260dc149f6367]::ty::Predicate>, (), core[20e502c22ca6166f]::hash::BuildHasherDefault<rustc_hash[c142d14f14395d39]::FxHasher>>>::insert ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#64 0x00007ff8093f0df6 in <rustc_data_structures[8c44cb2cf75f2b5c]::sso::map::SsoHashMap<rustc_infer[f1d617581dd2c354]::traits::Obligation<rustc_middle[ebf260dc149f6367]::ty::Predicate>, ()>>::insert ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#65 0x00007ff8093cff49 in <alloc[6644710a05fa25bc]::vec::drain_filter::DrainFilter<rustc_infer[f1d617581dd2c354]::traits::Obligation<rustc_middle[ebf260dc149f6367]::ty::Predicate>, rustc_trait_selection[7ec6d0943e96810b]::traits::project::opt_normalize_projection_type::{closure#0}> as core[20e502c22ca6166f]::iter::traits::iterator::Iterator>::next ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#66 0x00007ff8093c43fb in core[20e502c22ca6166f]::ptr::drop_in_place::<alloc[6644710a05fa25bc]::vec::drain_filter::DrainFilter<rustc_infer[f1d617581dd2c354]::traits::Obligation<rustc_middle[ebf260dc149f6367]::ty::Predicate>, rustc_trait_selection[7ec6d0943e96810b]::traits::project::opt_normalize_projection_type::{closure#0}>> () from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#67 0x00007ff80925d9ab in rustc_trait_selection[7ec6d0943e96810b]::traits::project::opt_normalize_projection_type ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#68 0x00007ff809254fbb in <rustc_trait_selection[7ec6d0943e96810b]::traits::project::AssocTypeNormalizer as rustc_type_ir[514e0b83b1a29ee1]::fold::TypeFolder<rustc_middle[ebf260dc149f6367]::ty::context::TyCtxt>>::fold_ty ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#69 0x00007ff8093aaa9b in <rustc_middle[ebf260dc149f6367]::ty::Ty as rustc_type_ir[514e0b83b1a29ee1]::fold::TypeSuperFoldable<rustc_middle[ebf260dc149f6367]::ty::context::TyCtxt>>::super_fold_with::<rustc_trait_selection[7ec6d0943e96810b]::traits::project::AssocTypeNormalizer> ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#70 0x00007ff809254e74 in <rustc_trait_selection[7ec6d0943e96810b]::traits::project::AssocTypeNormalizer as rustc_type_ir[514e0b83b1a29ee1]::fold::TypeFolder<rustc_middle[ebf260dc149f6367]::ty::context::TyCtxt>>::fold_ty ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#71 0x00007ff8094030ef in rustc_middle[ebf260dc149f6367]::ty::util::fold_list::<rustc_trait_selection[7ec6d0943e96810b]::traits::project::AssocTypeNormalizer, rustc_middle[ebf260dc149f6367]::ty::Ty, <&rustc_middle[ebf260dc149f6367]::ty::list::List<rustc_middle[ebf260dc149f6367]::ty::Ty> as rustc_type_ir[514e0b83b1a29ee1]::fold::TypeFoldable<rustc_middle[ebf260dc149f6367]::ty::context::TyCtxt>>::try_fold_with<rustc_trait_selection[7ec6d0943e96810b]::traits::project::AssocTypeNormalizer>::{closure#0}> ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#72 0x00007ff8093aabd1 in <rustc_middle[ebf260dc149f6367]::ty::Ty as rustc_type_ir[514e0b83b1a29ee1]::fold::TypeSuperFoldable<rustc_middle[ebf260dc149f6367]::ty::context::TyCtxt>>::super_fold_with::<rustc_trait_selection[7ec6d0943e96810b]::traits::project::AssocTypeNormalizer> ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#73 0x00007ff809254e74 in <rustc_trait_selection[7ec6d0943e96810b]::traits::project::AssocTypeNormalizer as rustc_type_ir[514e0b83b1a29ee1]::fold::TypeFolder<rustc_middle[ebf260dc149f6367]::ty::context::TyCtxt>>::fold_ty ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#74 0x00007ff8092f8cca in <rustc_middle[ebf260dc149f6367]::ty::subst::GenericArg as rustc_type_ir[514e0b83b1a29ee1]::fold::TypeFoldable<rustc_middle[ebf260dc149f6367]::ty::context::TyCtxt>>::try_fold_with::<rustc_trait_selection[7ec6d0943e96810b]::traits::project::AssocTypeNormalizer> ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#75 0x00007ff8093b6783 in <&rustc_middle[ebf260dc149f6367]::ty::list::List<rustc_middle[ebf260dc149f6367]::ty::subst::GenericArg> as rustc_type_ir[514e0b83b1a29ee1]::fold::TypeFoldable<rustc_middle[ebf260dc149f6367]::ty::context::TyCtxt>>::try_fold_with::<rustc_trait_selection[7ec6d0943e96810b]::traits::project::AssocTypeNormalizer> () from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#76 0x00007ff809288e27 in <rustc_middle[ebf260dc149f6367]::ty::sty::AliasTy as rustc_type_ir[514e0b83b1a29ee1]::fold::TypeFoldable<rustc_middle[ebf260dc149f6367]::ty::context::TyCtxt>>::try_fold_with::<rustc_trait_selection[7ec6d0943e96810b]::traits::project::AssocTypeNormalizer> ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#77 0x00007ff8093a27b1 in <rustc_middle[ebf260dc149f6367]::ty::PredicateKind as rustc_type_ir[514e0b83b1a29ee1]::fold::TypeFoldable<rustc_middle[ebf260dc149f6367]::ty::context::TyCtxt>>::try_fold_with::<rustc_trait_selection[7ec6d0943e96810b]::traits::project::AssocTypeNormalizer> ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#78 0x00007ff80928f38c in <rustc_middle[ebf260dc149f6367]::ty::sty::Binder<rustc_middle[ebf260dc149f6367]::ty::PredicateKind> as rustc_type_ir[514e0b83b1a29ee1]::fold::TypeSuperFoldable<rustc_middle[ebf260dc149f6367]::ty::context::TyCtxt>>::super_fold_with::<rustc_trait_selection[7ec6d0943e96810b]::traits::project::AssocTypeNormalizer> ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#79 0x00007ff80933cd22 in <rustc_trait_selection[7ec6d0943e96810b]::traits::project::AssocTypeNormalizer as rustc_type_ir[514e0b83b1a29ee1]::fold::FallibleTypeFolder<rustc_middle[ebf260dc149f6367]::ty::context::TyCtxt>>::try_fold_binder::<rustc_middle[ebf260dc149f6367]::ty::PredicateKind> ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#80 0x00007ff8093ac506 in <rustc_middle[ebf260dc149f6367]::ty::Predicate as rustc_type_ir[514e0b83b1a29ee1]::fold::TypeSuperFoldable<rustc_middle[ebf260dc149f6367]::ty::context::TyCtxt>>::super_fold_with::<rustc_trait_selection[7ec6d0943e96810b]::traits::project::AssocTypeNormalizer> ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#81 0x00007ff809345426 in <rustc_trait_selection[7ec6d0943e96810b]::traits::project::AssocTypeNormalizer>::fold::<rustc_middle[ebf260dc149f6367]::ty::Predicate> () from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#82 0x00007ff809348c9e in rustc_trait_selection[7ec6d0943e96810b]::traits::project::normalize_with_depth_to::<rustc_middle[ebf260dc149f6367]::ty::Predicate>
    () from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#83 0x00007ff809284233 in <rustc_trait_selection[7ec6d0943e96810b]::traits::select::SelectionContext>::impl_or_trait_obligations ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#84 0x00007ff809263794 in <rustc_trait_selection[7ec6d0943e96810b]::traits::select::SelectionContext>::vtable_impl ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#85 0x00007ff80927284e in <rustc_trait_selection[7ec6d0943e96810b]::traits::select::SelectionContext>::confirm_candidate ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
---Collapsed 2458 repeating stack frames---
#2543 0x00007ff80927a617 in <rustc_trait_selection[7ec6d0943e96810b]::traits::select::SelectionContext>::select ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2544 0x00007ff8092d4e55 in <rustc_infer[f1d617581dd2c354]::infer::InferCtxt>::commit_if_ok::<(), (), rustc_trait_selection[7ec6d0943e96810b]::traits::project::assemble_candidates_from_impls::{closure#0}> ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2545 0x00007ff80925f2a7 in rustc_trait_selection[7ec6d0943e96810b]::traits::project::opt_normalize_projection_type ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2546 0x00007ff809254fbb in <rustc_trait_selection[7ec6d0943e96810b]::traits::project::AssocTypeNormalizer as rustc_type_ir[514e0b83b1a29ee1]::fold::TypeFolder<rustc_middle[ebf260dc149f6367]::ty::context::TyCtxt>>::fold_ty ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2547 0x00007ff8093aaa9b in <rustc_middle[ebf260dc149f6367]::ty::Ty as rustc_type_ir[514e0b83b1a29ee1]::fold::TypeSuperFoldable<rustc_middle[ebf260dc149f6367]::ty::context::TyCtxt>>::super_fold_with::<rustc_trait_selection[7ec6d0943e96810b]::traits::project::AssocTypeNormalizer> ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2548 0x00007ff809254e74 in <rustc_trait_selection[7ec6d0943e96810b]::traits::project::AssocTypeNormalizer as rustc_type_ir[514e0b83b1a29ee1]::fold::TypeFolder<rustc_middle[ebf260dc149f6367]::ty::context::TyCtxt>>::fold_ty ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2549 0x00007ff8094030ef in rustc_middle[ebf260dc149f6367]::ty::util::fold_list::<rustc_trait_selection[7ec6d0943e96810b]::traits::project::AssocTypeNormalizer, rustc_middle[ebf260dc149f6367]::ty::Ty, <&rustc_middle[ebf260dc149f6367]::ty::list::List<rustc_middle[ebf260dc149f6367]::ty::Ty> as rustc_type_ir[514e0b83b1a29ee1]::fold::TypeFoldable<rustc_middle[ebf260dc149f6367]::ty::context::TyCtxt>>::try_fold_with<rustc_trait_selection[7ec6d0943e96810b]::traits::project::AssocTypeNormalizer>::{closure#0}> ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2550 0x00007ff8093aabd1 in <rustc_middle[ebf260dc149f6367]::ty::Ty as rustc_type_ir[514e0b83b1a29ee1]::fold::TypeSuperFoldable<rustc_middle[ebf260dc149f6367]::ty::context::TyCtxt>>::super_fold_with::<rustc_trait_selection[7ec6d0943e96810b]::traits::project::AssocTypeNormalizer> ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2551 0x00007ff809254e74 in <rustc_trait_selection[7ec6d0943e96810b]::traits::project::AssocTypeNormalizer as rustc_type_ir[514e0b83b1a29ee1]::fold::TypeFolder<rustc_middle[ebf260dc149f6367]::ty::context::TyCtxt>>::fold_ty ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2552 0x00007ff8092f8cca in <rustc_middle[ebf260dc149f6367]::ty::subst::GenericArg as rustc_type_ir[514e0b83b1a29ee1]::fold::TypeFoldable<rustc_middle[ebf260dc149f6367]::ty::context::TyCtxt>>::try_fold_with::<rustc_trait_selection[7ec6d0943e96810b]::traits::project::AssocTypeNormalizer> ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2553 0x00007ff8093b6783 in <&rustc_middle[ebf260dc149f6367]::ty::list::List<rustc_middle[ebf260dc149f6367]::ty::subst::GenericArg> as rustc_type_ir[514e0b83b1a29ee1]::fold::TypeFoldable<rustc_middle[ebf260dc149f6367]::ty::context::TyCtxt>>::try_fold_with::<rustc_trait_selection[7ec6d0943e96810b]::traits::project::AssocTypeNormalizer> ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2554 0x00007ff809289287 in <rustc_middle[ebf260dc149f6367]::ty::sty::TraitRef as rustc_type_ir[514e0b83b1a29ee1]::fold::TypeFoldable<rustc_middle[ebf260dc149f6367]::ty::context::TyCtxt>>::try_fold_with::<rustc_trait_selection[7ec6d0943e96810b]::traits::project::AssocTypeNormalizer> ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2555 0x00007ff8093a2573 in <rustc_middle[ebf260dc149f6367]::ty::PredicateKind as rustc_type_ir[514e0b83b1a29ee1]::fold::TypeFoldable<rustc_middle[ebf260dc149f6367]::ty::context::TyCtxt>>::try_fold_with::<rustc_trait_selection[7ec6d0943e96810b]::traits::project::AssocTypeNormalizer> ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2556 0x00007ff80928f38c in <rustc_middle[ebf260dc149f6367]::ty::sty::Binder<rustc_middle[ebf260dc149f6367]::ty::PredicateKind> as rustc_type_ir[514e0b83b1a29ee1]::fold::TypeSuperFoldable<rustc_middle[ebf260dc149f6367]::ty::context::TyCtxt>>::super_fold_with::<rustc_trait_selection[7ec6d0943e96810b]::traits::project::AssocTypeNormalizer> ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2557 0x00007ff80933cd22 in <rustc_trait_selection[7ec6d0943e96810b]::traits::project::AssocTypeNormalizer as rustc_type_ir[514e0b83b1a29ee1]::fold::FallibleTypeFolder<rustc_middle[ebf260dc149f6367]::ty::context::TyCtxt>>::try_fold_binder::<rustc_middle[ebf260dc149f6367]::ty::PredicateKind> ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2558 0x00007ff8093ac506 in <rustc_middle[ebf260dc149f6367]::ty::Predicate as rustc_type_ir[514e0b83b1a29ee1]::fold::TypeSuperFoldable<rustc_middle[ebf260dc149f6367]::ty::context::TyCtxt>>::super_fold_with::<rustc_trait_selection[7ec6d0943e96810b]::traits::project::AssocTypeNormalizer> ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2559 0x00007ff80933d7e6 in <rustc_trait_selection[7ec6d0943e96810b]::traits::project::AssocTypeNormalizer as rustc_type_ir[514e0b83b1a29ee1]::fold::FallibleTypeFolder<rustc_middle[ebf260dc149f6367]::ty::context::TyCtxt>>::try_fold_predicate () from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2560 0x00007ff809366feb in <core[20e502c22ca6166f]::iter::adapters::map::Map<alloc[6644710a05fa25bc]::vec::into_iter::IntoIter<rustc_middle[ebf260dc149f6367]::ty::Predicate>, <alloc[6644710a05fa25bc]::vec::Vec<rustc_middle[ebf260dc149f6367]::ty::Predicate> as rustc_type_ir[514e0b83b1a29ee1]::fold::TypeFoldable<rustc_middle[ebf260dc149f6367]::ty::context::TyCtxt>>::try_fold_with<rustc_trait_selection[7ec6d0943e96810b]::traits::project::AssocTypeNormalizer>::{closure#0}> as core[20e502c22ca6166f]::iter::traits::iterator::Iterator>::try_fold::<alloc[6644710a05fa25bc]::vec::in_place_drop::InPlaceDrop<rustc_middle[ebf260dc149f6367]::ty::Predicate>, <core[20e502c22ca6166f]::iter::adapters::GenericShunt<core[20e502c22ca6166f]::iter::adapters::map::Map<alloc[6644710a05fa25bc]::vec::into_iter::IntoIter<rustc_middle[ebf260dc149f6367]::ty::Predicate>, <alloc[6644710a05fa25bc]::vec::Vec<rustc_middle[ebf260dc149f6367]::ty::Predicate> as rustc_type_ir[514e0b83b1a29ee1]::fold::TypeFoldable<rustc_middle[ebf260dc149f6367]::ty::context::TyCtxt>>::try_fold_with<rustc_trait_selection[7ec6d0943e96810b]::traits::project::AssocTypeNormalizer>::{closure#0}>, core[20e502c22ca6166f]::result::Result<core[20e502c22ca6166f]::convert::Infallible, !>> as core[20e502c22ca6166f]::iter::traits::iterator::Iterator>::try_fold<alloc[6644710a05fa25bc]::vec::in_place_drop::InPlaceDrop<rustc_middle[ebf260dc149f6367]::ty::Predicate>, alloc[6644710a05fa25bc]::vec::in_place_collect::write_in_place_with_drop<rustc_middle[ebf260dc149f6367]::ty::Predicate>::{closure#0}, core[20e502c22ca6166f]::result::Result<alloc[6644710a05fa25bc]::vec::in_place_drop::InPlaceDrop<rustc_middle[ebf260dc149f6367]::ty::Predicate>, !>>::{closure#0}, core[20e502c22ca6166f]::ops::control_flow::ControlFlow<core[20e502c22ca6166f]::result::Result<alloc[6644710a05fa25bc]::vec::in_place_drop::InPlaceDrop<rustc_middle[ebf260dc149f6367]::ty::Predicate>, !>, alloc[6644710a05fa25bc]::vec::in_place_drop::InPlaceDrop<rustc_middle[ebf260dc149f6367]::ty::Predicate>>> ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2561 0x00007ff80940d3e2 in <core[20e502c22ca6166f]::iter::adapters::GenericShunt<core[20e502c22ca6166f]::iter::adapters::map::Map<alloc[6644710a05fa25bc]::vec::into_iter::IntoIter<rustc_middle[ebf260dc149f6367]::ty::Predicate>, <alloc[6644710a05fa25bc]::vec::Vec<rustc_middle[ebf260dc149f6367]::ty::Predicate> as rustc_type_ir[514e0b83b1a29ee1]::fold::TypeFoldable<rustc_middle[ebf260dc149f6367]::ty::context::TyCtxt>>::try_fold_with<rustc_trait_selection[7ec6d0943e96810b]::traits::project::AssocTypeNormalizer>::{closure#0}>, core[20e502c22ca6166f]::result::Result<core[20e502c22ca6166f]::convert::Infallible, !>> as core[20e502c22ca6166f]::iter::traits::iterator::Iterator>::try_fold::<alloc[6644710a05fa25bc]::vec::in_place_drop::InPlaceDrop<rustc_middle[ebf260dc149f6367]::ty::Predicate>, alloc[6644710a05fa25bc]::vec::in_place_collect::write_in_place_with_drop<rustc_middle[ebf260dc149f6367]::ty::Predicate>::{closure#0}, core[20e502c22ca6166f]::result::Result<alloc[6644710a05fa25bc]::vec::in_place_drop::InPlaceDrop<rustc_middle[ebf260dc149f6367]::ty::Predicate>, !>> ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2562 0x00007ff8092a67ea in <alloc[6644710a05fa25bc]::vec::Vec<rustc_middle[ebf260dc149f6367]::ty::Predicate> as alloc[6644710a05fa25bc]::vec::spec_from_iter::SpecFromIter<rustc_middle[ebf260dc149f6367]::ty::Predicate, core[20e502c22ca6166f]::iter::adapters::GenericShunt<core[20e502c22ca6166f]::iter::adapters::map::Map<alloc[6644710a05fa25bc]::vec::into_iter::IntoIter<rustc_middle[ebf260dc149f6367]::ty::Predicate>, <alloc[6644710a05fa25bc]::vec::Vec<rustc_middle[ebf260dc149f6367]::ty::Predicate> as rustc_type_ir[514e0b83b1a29ee1]::fold::TypeFoldable<rustc_middle[ebf260dc149f6367]::ty::context::TyCtxt>>::try_fold_with<rustc_trait_selection[7ec6d0943e96810b]::traits::project::AssocTypeNormalizer>::{closure#0}>, core[20e502c22ca6166f]::result::Result<core[20e502c22ca6166f]::convert::Infallible, !>>>>::from_iter ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2563 0x00007ff80940cdeb in core[20e502c22ca6166f]::iter::adapters::try_process::<core[20e502c22ca6166f]::iter::adapters::map::Map<alloc[6644710a05fa25bc]::vec::into_iter::IntoIter<rustc_middle[ebf260dc149f6367]::ty::Predicate>, <alloc[6644710a05fa25bc]::vec::Vec<rustc_middle[ebf260dc149f6367]::ty::Predicate> as rustc_type_ir[514e0b83b1a29ee1]::fold::TypeFoldable<rustc_middle[ebf260dc149f6367]::ty::context::TyCtxt>>::try_fold_with<rustc_trait_selection[7ec6d0943e96810b]::traits::project::AssocTypeNormalizer>::{closure#0}>, rustc_middle[ebf260dc149f6367]::ty::Predicate, core[20e502c22ca6166f]::result::Result<core[20e502c22ca6166f]::convert::Infallible, !>, <core[20e502c22ca6166f]::result::Result<alloc[6644710a05fa25bc]::vec::Vec<rustc_middle[ebf260dc149f6367]::ty::Predicate>, !> as core[20e502c22ca6166f]::iter::traits::collect::FromIterator<core[20e502c22ca6166f]::result::Result<rustc_middle[ebf260dc149f6367]::ty::Predicate, !>>>::from_iter<core[20e502c22ca6166f]::iter::adapters::map::Map<alloc[6644710a05fa25bc]::vec::into_iter::IntoIter<rustc_middle[ebf260dc149f6367]::ty::Predicate>, <alloc[6644710a05fa25bc]::vec::Vec<rustc_middle[ebf260dc149f6367]::ty::Predicate> as rustc_type_ir[514e0b83b1a29ee1]::fold::TypeFoldable<rustc_middle[ebf260dc149f6367]::ty::context::TyCtxt>>::try_fold_with<rustc_trait_selection[7ec6d0943e96810b]::traits::project::AssocTypeNormalizer>::{closure#0}>>::{closure#0}, alloc[6644710a05fa25bc]::vec::Vec<rustc_middle[ebf260dc149f6367]::ty::Predicate>> ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2564 0x00007ff80929826f in <alloc[6644710a05fa25bc]::vec::Vec<rustc_middle[ebf260dc149f6367]::ty::Predicate> as rustc_type_ir[514e0b83b1a29ee1]::fold::TypeFoldable<rustc_middle[ebf260dc149f6367]::ty::context::TyCtxt>>::fold_with::<rustc_trait_selection[7ec6d0943e96810b]::traits::project::AssocTypeNormalizer> ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2565 0x00007ff809343894 in <rustc_trait_selection[7ec6d0943e96810b]::traits::project::AssocTypeNormalizer>::fold::<alloc[6644710a05fa25bc]::vec::Vec<rustc_middle[ebf260dc149f6367]::ty::Predicate>> ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2566 0x00007ff80933de00 in rustc_trait_selection[7ec6d0943e96810b]::traits::project::normalize_with_depth::<alloc[6644710a05fa25bc]::vec::Vec<rustc_middle[ebf260dc149f6367]::ty::Predicate>> ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2567 0x00007ff8092b1e3c in <rustc_infer[f1d617581dd2c354]::infer::at::At as rustc_trait_selection[7ec6d0943e96810b]::traits::project::NormalizeExt>::normalize::<alloc[6644710a05fa25bc]::vec::Vec<rustc_middle[ebf260dc149f6367]::ty::Predicate>> () from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2568 0x00007ff80946a178 in <rustc_trait_selection[7ec6d0943e96810b]::traits::engine::ObligationCtxt>::normalize::<alloc[6644710a05fa25bc]::vec::Vec<rustc_middle[ebf260dc149f6367]::ty::Predicate>> ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2569 0x00007ff80946c72b in rustc_trait_selection[7ec6d0943e96810b]::traits::fully_normalize::<alloc[6644710a05fa25bc]::vec::Vec<rustc_middle[ebf260dc149f6367]::ty::Predicate>> () from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2570 0x00007ff809410066 in rustc_trait_selection[7ec6d0943e96810b]::traits::do_normalize_predicates ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2571 0x00007ff80941126f in rustc_trait_selection[7ec6d0943e96810b]::traits::normalize_param_env_or_error ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2572 0x00007ff8073cf44f in rustc_ty_utils[4a5eeb4403464e8c]::ty::param_env ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2573 0x00007ff8089277f3 in rustc_query_system[f1ff4f506666fc4d]::query::plumbing::try_execute_query::<rustc_query_impl[f865b387ba6e061a]::queries::param_env, rustc_query_impl[f865b387ba6e061a]::plumbing::QueryCtxt> ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2574 0x00007ff8089b9a47 in <rustc_query_impl[f865b387ba6e061a]::Queries as rustc_middle[ebf260dc149f6367]::ty::query::QueryEngine>::param_env ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2575 0x00007ff8078650c4 in rustc_hir_analysis[25229fcfe87c7597]::check::wfcheck::enter_wf_checking_ctxt::<rustc_hir_analysis[25229fcfe87c7597]::check::wfcheck::check_impl::{closure#0}> ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2576 0x00007ff807751bc3 in rustc_hir_analysis[25229fcfe87c7597]::check::wfcheck::check_well_formed ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2577 0x00007ff808847295 in rustc_query_system[f1ff4f506666fc4d]::query::plumbing::try_execute_query::<rustc_query_impl[f865b387ba6e061a]::queries::check_well_formed, rustc_query_impl[f865b387ba6e061a]::plumbing::QueryCtxt> ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2578 0x00007ff8089c7506 in <rustc_query_impl[f865b387ba6e061a]::Queries as rustc_middle[ebf260dc149f6367]::ty::query::QueryEngine>::check_well_formed ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2579 0x00007ff8078800b9 in <core[20e502c22ca6166f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[8c44cb2cf75f2b5c]::sync::par_for_each_in<&[rustc_hir[9f049f9ca34b43c2]::hir::ItemId], <rustc_middle[ebf260dc149f6367]::hir::ModuleItems>::par_items<rustc_hir_analysis[25229fcfe87c7597]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[20e502c22ca6166f]::ops::function::FnOnce<()>>::call_once ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2580 0x00007ff807870279 in std[6f73a53b78cbdd15]::panicking::try::<(), core[20e502c22ca6166f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[8c44cb2cf75f2b5c]::sync::par_for_each_in<&[rustc_hir[9f049f9ca34b43c2]::hir::ItemId], <rustc_middle[ebf260dc149f6367]::hir::ModuleItems>::par_items<rustc_hir_analysis[25229fcfe87c7597]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>> ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2581 0x00007ff80776dfb5 in rustc_data_structures[8c44cb2cf75f2b5c]::sync::par_for_each_in::<&[rustc_hir[9f049f9ca34b43c2]::hir::ItemId], <rustc_middle[ebf260dc149f6367]::hir::ModuleItems>::par_items<rustc_hir_analysis[25229fcfe87c7597]::check::wfcheck::check_mod_type_wf::{closure#0}>::{closure#0}> ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2582 0x00007ff8077547b8 in rustc_hir_analysis[25229fcfe87c7597]::check::wfcheck::check_mod_type_wf ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2583 0x00007ff808846359 in rustc_query_system[f1ff4f506666fc4d]::query::plumbing::try_execute_query::<rustc_query_impl[f865b387ba6e061a]::queries::check_mod_type_wf, rustc_query_impl[f865b387ba6e061a]::plumbing::QueryCtxt> ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2584 0x00007ff80899d776 in <rustc_query_impl[f865b387ba6e061a]::Queries as rustc_middle[ebf260dc149f6367]::ty::query::QueryEngine>::check_mod_type_wf ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2585 0x00007ff8078801b9 in <core[20e502c22ca6166f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[8c44cb2cf75f2b5c]::sync::par_for_each_in<&[rustc_hir[9f049f9ca34b43c2]::hir_id::OwnerId], <rustc_middle[ebf260dc149f6367]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[25229fcfe87c7597]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[20e502c22ca6166f]::ops::function::FnOnce<()>>::call_once ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2586 0x00007ff807870299 in std[6f73a53b78cbdd15]::panicking::try::<(), core[20e502c22ca6166f]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[8c44cb2cf75f2b5c]::sync::par_for_each_in<&[rustc_hir[9f049f9ca34b43c2]::hir_id::OwnerId], <rustc_middle[ebf260dc149f6367]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[25229fcfe87c7597]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>> ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2587 0x00007ff80776e085 in rustc_data_structures[8c44cb2cf75f2b5c]::sync::par_for_each_in::<&[rustc_hir[9f049f9ca34b43c2]::hir_id::OwnerId], <rustc_middle[ebf260dc149f6367]::hir::map::Map>::par_for_each_module<rustc_hir_analysis[25229fcfe87c7597]::check_crate::{closure#5}::{closure#0}::{closure#0}>::{closure#0}>
    () from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2588 0x00007ff80780f0dc in <rustc_session[94c1e885e8c82419]::session::Session>::track_errors::<rustc_hir_analysis[25229fcfe87c7597]::check_crate::{closure#5}, ()> () from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2589 0x00007ff80784bb93 in rustc_hir_analysis[25229fcfe87c7597]::check_crate
    () from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2590 0x00007ff8071e7d41 in rustc_interface[1496720bf34f4141]::passes::analysis () from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2591 0x00007ff8089169b1 in rustc_query_system[f1ff4f506666fc4d]::query::plumbing::try_execute_query::<rustc_query_impl[f865b387ba6e061a]::queries::analysis, rustc_query_impl[f865b387ba6e061a]::plumbing::QueryCtxt> ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2592 0x00007ff808976882 in <rustc_query_impl[f865b387ba6e061a]::Queries as rustc_middle[ebf260dc149f6367]::ty::query::QueryEngine>::analysis ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2593 0x00007ff8071714ea in <rustc_middle[ebf260dc149f6367]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[137639c14726341]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[20e502c22ca6166f]::result::Result<(), rustc_span[65f57099d4b96856]::ErrorGuaranteed>> ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2594 0x00007ff80716d8fe in <rustc_interface[1496720bf34f4141]::interface::Compiler>::enter::<rustc_driver_impl[137639c14726341]::run_compiler::{closure#1}::{closure#2}, core[20e502c22ca6166f]::result::Result<core[20e502c22ca6166f]::option::Option<rustc_interface[1496720bf34f4141]::queries::Linker>, rustc_span[65f57099d4b96856]::ErrorGuaranteed>> ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2595 0x00007ff807160a96 in rustc_span[65f57099d4b96856]::with_source_map::<core[20e502c22ca6166f]::result::Result<(), rustc_span[65f57099d4b96856]::ErrorGuaranteed>, rustc_interface[1496720bf34f4141]::interface::run_compiler<core[20e502c22ca6166f]::result::Result<(), rustc_span[65f57099d4b96856]::ErrorGuaranteed>, rustc_driver_impl[137639c14726341]::run_compiler::{closure#1}>::{closure#0}::{closure#0}> () from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2596 0x00007ff807157387 in <scoped_tls[c8b2a3d21b60c86d]::ScopedKey<rustc_span[65f57099d4b96856]::SessionGlobals>>::set::<rustc_interface[1496720bf34f4141]::interface::run_compiler<core[20e502c22ca6166f]::result::Result<(), rustc_span[65f57099d4b96856]::ErrorGuaranteed>, rustc_driver_impl[137639c14726341]::run_compiler::{closure#1}>::{closure#0}, core[20e502c22ca6166f]::result::Result<(), rustc_span[65f57099d4b96856]::ErrorGuaranteed>> ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2597 0x00007ff807171cfd in std[6f73a53b78cbdd15]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[1496720bf34f4141]::util::run_in_thread_pool_with_globals<rustc_interface[1496720bf34f4141]::interface::run_compiler<core[20e502c22ca6166f]::result::Result<(), rustc_span[65f57099d4b96856]::ErrorGuaranteed>, rustc_driver_impl[137639c14726341]::run_compiler::{closure#1}>::{closure#0}, core[20e502c22ca6166f]::result::Result<(), rustc_span[65f57099d4b96856]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[20e502c22ca6166f]::result::Result<(), rustc_span[65f57099d4b96856]::ErrorGuaranteed>> ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2598 0x00007ff80716ce31 in std[6f73a53b78cbdd15]::panicking::try::<core[20e502c22ca6166f]::result::Result<(), rustc_span[65f57099d4b96856]::ErrorGuaranteed>, core[20e502c22ca6166f]::panic::unwind_safe::AssertUnwindSafe<<std[6f73a53b78cbdd15]::thread::Builder>::spawn_unchecked_<rustc_interface[1496720bf34f4141]::util::run_in_thread_pool_with_globals<rustc_interface[1496720bf34f4141]::interface::run_compiler<core[20e502c22ca6166f]::result::Result<(), rustc_span[65f57099d4b96856]::ErrorGuaranteed>, rustc_driver_impl[137639c14726341]::run_compiler::{closure#1}>::{closure#0}, core[20e502c22ca6166f]::result::Result<(), rustc_span[65f57099d4b96856]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[20e502c22ca6166f]::result::Result<(), rustc_span[65f57099d4b96856]::ErrorGuaranteed>>::{closure#1}::{closure#0}>> ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2599 0x00007ff807174282 in <<std[6f73a53b78cbdd15]::thread::Builder>::spawn_unchecked_<rustc_interface[1496720bf34f4141]::util::run_in_thread_pool_with_globals<rustc_interface[1496720bf34f4141]::interface::run_compiler<core[20e502c22ca6166f]::result::Result<(), rustc_span[65f57099d4b96856]::ErrorGuaranteed>, rustc_driver_impl[137639c14726341]::run_compiler::{closure#1}>::{closure#0}, core[20e502c22ca6166f]::result::Result<(), rustc_span[65f57099d4b96856]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[20e502c22ca6166f]::result::Result<(), rustc_span[65f57099d4b96856]::ErrorGuaranteed>>::{closure#1} as core[20e502c22ca6166f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0} ()
   from C:\usr\local\bin\rustc_driver-eda9524c4e602df3.dll
#2600 0x00007ff83375da69 in <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h7fd5a5c69811f34a ()
   from C:\usr\local\bin\std-a84f40a7e9b3ae6c.dll
