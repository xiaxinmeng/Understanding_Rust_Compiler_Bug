
243:                                              ; preds = %259
  %244 = landingpad { i8*, i32 }
          cleanup
  br label %247

245:                                              ; preds = %227
  %246 = landingpad { i8*, i32 }
          cleanup
  br label %247

247:                                              ; preds = %245, %243
  %248 = phi { i8*, i32 } [ %244, %243 ], [ %246, %245 ]
  invoke fastcc void @"_ZN4core3ptr101drop_in_place$LT$rustc_ast_lowering..lifetimes_from_impl_trait_bounds..ImplTraitLifetimeCollector$GT$17hde8a472161ebd31bE"(%"lifetimes_from_impl_trait_bounds::ImplTraitLifetimeCollector"* nonnull %30) #33
          to label %749 unwind label %570
