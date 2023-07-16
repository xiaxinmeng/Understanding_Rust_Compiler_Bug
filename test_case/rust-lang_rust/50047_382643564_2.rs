
target datalayout = "e-m:e-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

define internal fastcc void @"<alloc::alloc::Global as core::alloc::GlobalAlloc>::oom"() unnamed_addr #0 {
  tail call void @__rust_oom()
  unreachable
}

define noalias align 1 dereferenceable(8) [8 x i8]* @example::foo() unnamed_addr #1 {
  %0 = tail call i8* @__rust_alloc(i64 8, i64 1) #5
  %1 = icmp eq i8* %0, null
  br i1 %1, label %bb7.i.i, label %"_ZN35_$LT$alloc..boxed..Box$LT$T$GT$$GT$3new17h4e4edfda70298278E.exit"

bb7.i.i: ; preds = %start
  tail call fastcc void @"<alloc::alloc::Global as core::alloc::GlobalAlloc>::oom"() #5
  unreachable

"_ZN35_$LT$alloc..boxed..Box$LT$T$GT$$GT$3new17h4e4edfda70298278E.exit": ; preds = %start
  %2 = bitcast i8* %0 to [8 x i8]*
  %_3.sroa.0.0..sroa_cast.i = bitcast i8* %0 to i64*
  store i64 0, i64* %_3.sroa.0.0..sroa_cast.i, align 1
  ret [8 x i8]* %2
}

define noalias align 1 dereferenceable(9) [9 x i8]* @example::bar() unnamed_addr #1 {
  %_1 = alloca [9 x i8], align 1
  %_1.0.sroa_idx2 = getelementptr inbounds [9 x i8], [9 x i8]* %_1, i64 0, i64 0
  call void @llvm.lifetime.start.p0i8(i64 9, i8* nonnull %_1.0.sroa_idx2)
  call void @llvm.memset.p0i8.i64(i8* nonnull %_1.0.sroa_idx2, i8 0, i64 9, i32 1, i1 false)
  %0 = tail call i8* @__rust_alloc(i64 9, i64 1) #5, !noalias !0
  %1 = icmp eq i8* %0, null
  br i1 %1, label %bb7.i.i, label %"_ZN35_$LT$alloc..boxed..Box$LT$T$GT$$GT$3new17ha687450047947beaE.exit"

bb7.i.i: ; preds = %start
  tail call fastcc void @"<alloc::alloc::Global as core::alloc::GlobalAlloc>::oom"() #5, !noalias !0
  unreachable

"_ZN35_$LT$alloc..boxed..Box$LT$T$GT$$GT$3new17ha687450047947beaE.exit": ; preds = %start
  %2 = bitcast i8* %0 to [9 x i8]*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* nonnull %0, i8* nonnull %_1.0.sroa_idx2, i64 9, i32 1, i1 false) #5
  call void @llvm.lifetime.end.p0i8(i64 9, i8* nonnull %_1.0.sroa_idx2)
  ret [9 x i8]* %2
}

declare void @llvm.memcpy.p0i8.p0i8.i64(i8* nocapture writeonly, i8* nocapture readonly, i64, i32, i1) #2

declare void @__rust_oom() unnamed_addr #3

declare noalias i8* @__rust_alloc(i64, i64) unnamed_addr #4

declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i32, i1) #2

declare void @llvm.lifetime.start.p0i8(i64, i8* nocapture) #2

declare void @llvm.lifetime.end.p0i8(i64, i8* nocapture) #2

attributes #0 = { inlinehint noreturn nounwind uwtable "probe-stack"="__rust_probestack" }
attributes #1 = { nounwind uwtable "probe-stack"="__rust_probestack" }
attributes #2 = { argmemonly nounwind }
attributes #3 = { cold noreturn nounwind "probe-stack"="__rust_probestack" }
attributes #4 = { nounwind "probe-stack"="__rust_probestack" }
attributes #5 = { nounwind }

!0 = !{!1}
!1 = distinct !{!1, !2, !"<alloc::boxed::Box<T>>::new: %x"}
!2 = distinct !{!2, !"<alloc::boxed::Box<T>>::new"}
