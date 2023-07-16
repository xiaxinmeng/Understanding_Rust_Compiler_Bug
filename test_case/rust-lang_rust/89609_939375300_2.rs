
106767:                                           ; preds = %754032
  %106768 = landingpad { i8*, i32 }
          cleanup
  br label %106769

106769:                                           ; preds = %106767, %106765, ...etc.
  %106770 = phi { i8*, i32 } [ %86006, %86005 ], [ %86008, %86007 ], ...etc.
  invoke fastcc void @"_ZN4core3ptr101drop_in_place$LT$rustc_ast_lowering..lifetimes_from_impl_trait_bounds..ImplTraitLifetimeCollector$GT$17hde8a472161ebd31bE"(%"lifetimes_from_impl_trait_bounds::ImplTraitLifetimeCollector"* nonnull %15650) #25
          to label %754701 unwind label %754477
