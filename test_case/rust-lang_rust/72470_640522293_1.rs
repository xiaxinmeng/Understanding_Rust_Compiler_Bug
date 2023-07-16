
115:                                              ; preds = %61, %59, %57
  %116 = phi i8* [ %44, %61 ], [ %44, %59 ], [ %53, %57 ]
  %117 = phi %"core::future::from_generator::GenFuture<llvm_ice::foo::{{closure}}<core::future::from_generator::GenFuture<llvm_ice::run::{{closure}}::{{closure}}::{{closure}}>, llvm_ice::run::{{closure}}::{{closure}}>>.0"* [ %24, %61 ], [ %24, %59 ], [ %55, %57 ]
  %118 = phi i8* [ %21, %61 ], [ %21, %59 ], [ %54, %57 ]
  %119 = cleanuppad within none []
  call void @llvm.lifetime.end.p0i8(i64 24, i8* nonnull %116)
  cleanupret from %119 unwind label %120

120:                                              ; preds = %115, %112, %106
  %121 = phi %"core::future::from_generator::GenFuture<llvm_ice::foo::{{closure}}<core::future::from_generator::GenFuture<llvm_ice::run::{{closure}}::{{closure}}::{{closure}}>, llvm_ice::run::{{closure}}::{{closure}}>>.0"* [ %117, %115 ], [ %84, %112 ], [ %84, %106 ]
  %122 = phi i8* [ %118, %115 ], [ %85, %112 ], [ %85, %106 ]
  %123 = cleanuppad within none []
  %124 = getelementptr inbounds [24 x i8], [24 x i8]* %9, i64 0, i64 0
  call void @llvm.lifetime.end.p0i8(i64 24, i8* nonnull %124)
  %125 = getelementptr inbounds [7 x i8], [7 x i8]* %10, i64 0, i64 0
  call void @llvm.lifetime.end.p0i8(i64 7, i8* nonnull %125)
  store i8 2, i8* %122, align 8
  cleanupret from %123 unwind label %254
