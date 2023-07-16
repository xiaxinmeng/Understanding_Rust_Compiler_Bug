
; Function Attrs: uwtable
define void @_ZN5slice20h3a3095f0767f7252daaE(%str_slice* noalias nocapture sret dereferenceable(16), %str_slice* noalias nocapture dereferenceable(16), i64, i64) unnamed_addr #0 {
entry-block:
  %arg3.i = alloca %str_slice, align 8
  %4 = icmp ult i64 %3, %2
  br i1 %4, label %else-block.i, label %before_rhs.i

join.i:                                           ; preds = %next-block.i.i
  %5 = getelementptr inbounds %str_slice* %1, i64 0, i32 0
  %6 = load i8** %5, align 8
  %7 = getelementptr inbounds i8* %6, i64 %2
  %8 = load i8* %7, align 1
  %9 = icmp sgt i8 %8, -1
  %10 = icmp ugt i8 %8, -65
  %..i.i = or i1 %9, %10
  br i1 %..i.i, label %before_rhs2.i, label %else-block.i

before_rhs.i:                                     ; preds = %entry-block
  %11 = getelementptr inbounds %str_slice* %1, i64 0, i32 1
  %12 = load i64* %11, align 8
  %13 = icmp eq i64 %12, %2
  br i1 %13, label %before_rhs2.i, label %next-block.i.i

next-block.i.i:                                   ; preds = %before_rhs.i
  %14 = icmp ugt i64 %12, %2
  br i1 %14, label %join.i, label %else-block.i

join1.i:                                          ; preds = %next-block.i6.i
  %15 = getelementptr inbounds %str_slice* %1, i64 0, i32 0
  %16 = load i8** %15, align 8
  %17 = getelementptr inbounds i8* %16, i64 %3
  %18 = load i8* %17, align 1
  %19 = icmp sgt i8 %18, -1
  %20 = icmp ugt i8 %18, -65
  %..i7.i = or i1 %19, %20
  br i1 %..i7.i, label %"_ZN3str39_$BP$$x27a$x20str.StrSlice$LT$$x27a$GT$5slice20h40444d952ab6abe5AaaE.exit", label %else-block.i

before_rhs2.i:                                    ; preds = %before_rhs.i, %join.i
  %21 = icmp eq i64 %12, %3
  br i1 %21, label %before_rhs2.then-block-56-_crit_edge.i, label %next-block.i6.i

before_rhs2.then-block-56-_crit_edge.i:           ; preds = %before_rhs2.i
  %.phi.trans.insert.i = getelementptr inbounds %str_slice* %1, i64 0, i32 0
  %.pre.i = load i8** %.phi.trans.insert.i, align 8
  br label %"_ZN3str39_$BP$$x27a$x20str.StrSlice$LT$$x27a$GT$5slice20h40444d952ab6abe5AaaE.exit"

next-block.i6.i:                                  ; preds = %before_rhs2.i
  %22 = icmp ugt i64 %12, %3
  br i1 %22, label %join1.i, label %else-block.i

else-block.i:                                     ; preds = %next-block.i6.i, %join1.i, %next-block.i.i, %join.i, %entry-block
  %23 = bitcast %str_slice* %arg3.i to i8*
  call void @llvm.lifetime.start(i64 16, i8* %23)
  %24 = bitcast %str_slice* %1 to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %23, i8* %24, i64 16, i32 8, i1 false)
  call void @_ZN3str16slice_error_fail20h38ed0a1e33b8688ezvrE(%str_slice* noalias nocapture dereferenceable(16) %arg3.i, i64 %2, i64 %3)
  unreachable

"_ZN3str39_$BP$$x27a$x20str.StrSlice$LT$$x27a$GT$5slice20h40444d952ab6abe5AaaE.exit": ; preds = %join1.i, %before_rhs2.then-block-56-_crit_edge.i
  %25 = phi i8* [ %.pre.i, %before_rhs2.then-block-56-_crit_edge.i ], [ %16, %join1.i ]
  %26 = getelementptr inbounds %str_slice* %0, i64 0, i32 0
  %27 = getelementptr inbounds i8* %25, i64 %2
  store i8* %27, i8** %26, align 8
  %28 = getelementptr inbounds %str_slice* %0, i64 0, i32 1
  %29 = sub i64 %3, %2
  store i64 %29, i64* %28, align 8
  %30 = bitcast %str_slice* %1 to i8*
  tail call void @llvm.lifetime.end(i64 16, i8* %30)
  ret void
}
