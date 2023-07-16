
115:                                              ; preds = %57, %59, %61, %112, %106
  %116 = phi %"core::future::from_generator::GenFuture<llvm_ice::foo::{{closure}}<core::future::from_generator::GenFuture<llvm_ice::run::{{closure}}::{{closure}}::{{closure}}>, llvm_ice::run::{{closure}}::{{closure}}>>.0"* [ %84, %112 ], [ %84, %106 ], [ %24, %61 ], [ %24, %59 ], [ %55, %57 ]
  %117 = phi i8* [ %85, %112 ], [ %85, %106 ], [ %21, %61 ], [ %21, %59 ], [ %54, %57 ]
# %118 being referenced in its own definition?
  %118 = phi i8* [ %44, %61 ], [ %44, %59 ], [ %53, %57 ], [ %118, %112 ], [ %118, %106 ]
  %119 = cleanuppad within none []
  %120 = getelementptr inbounds [24 x i8], [24 x i8]* %9, i64 0, i64 0
  call void @llvm.lifetime.end.p0i8(i64 24, i8* nonnull %120)
  %121 = getelementptr inbounds [7 x i8], [7 x i8]* %10, i64 0, i64 0
  call void @llvm.lifetime.end.p0i8(i64 7, i8* nonnull %121)
  store i8 2, i8* %117, align 8
  cleanupret from %119 unwind label %246
