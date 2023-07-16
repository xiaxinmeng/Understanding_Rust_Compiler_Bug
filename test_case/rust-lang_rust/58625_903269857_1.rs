
define void @_ZN5test23foo17he3396acf5b2d9080E([1000 x [1000 x i32]]* noalias nocapture sret([1000 x [1000 x i32]]) dereferenceable(4000000) %0) unnamed_addr #0 {
start:
  %1 = getelementptr inbounds [1000 x [1000 x i32]], [1000 x [1000 x i32]]* %0, i64 0, i64 0
  %2 = getelementptr inbounds [1000 x [1000 x i32]], [1000 x [1000 x i32]]* %0, i64 0, i64 1000
  br label %repeat_loop_body

repeat_loop_body:                                 ; preds = %repeat_loop_body, %start
  %3 = phi [1000 x i32]* [ %1, %start ], [ %53, %repeat_loop_body ]
  %4 = bitcast [1000 x i32]* %3 to i8*
  call void @llvm.memset.p0i8.i64(i8* noundef nonnull align 4 dereferenceable(4000) %4, i8 0, i64 4000, i1 false)
  %5 = getelementptr inbounds [1000 x i32], [1000 x i32]* %3, i64 1
  %6 = bitcast [1000 x i32]* %5 to i8*
  call void @llvm.memset.p0i8.i64(i8* noundef nonnull align 4 dereferenceable(4000) %6, i8 0, i64 4000, i1 false)
  %7 = getelementptr inbounds [1000 x i32], [1000 x i32]* %3, i64 2
  %8 = bitcast [1000 x i32]* %7 to i8*
  call void @llvm.memset.p0i8.i64(i8* noundef nonnull align 4 dereferenceable(4000) %8, i8 0, i64 4000, i1 false)
  %9 = getelementptr inbounds [1000 x i32], [1000 x i32]* %3, i64 3
  %10 = bitcast [1000 x i32]* %9 to i8*
  call void @llvm.memset.p0i8.i64(i8* noundef nonnull align 4 dereferenceable(4000) %10, i8 0, i64 4000, i1 false)
  %11 = getelementptr inbounds [1000 x i32], [1000 x i32]* %3, i64 4
  %12 = bitcast [1000 x i32]* %11 to i8*
  call void @llvm.memset.p0i8.i64(i8* noundef nonnull align 4 dereferenceable(4000) %12, i8 0, i64 4000, i1 false)
  %13 = getelementptr inbounds [1000 x i32], [1000 x i32]* %3, i64 5
  %14 = bitcast [1000 x i32]* %13 to i8*
  call void @llvm.memset.p0i8.i64(i8* noundef nonnull align 4 dereferenceable(4000) %14, i8 0, i64 4000, i1 false)
  %15 = getelementptr inbounds [1000 x i32], [1000 x i32]* %3, i64 6
  %16 = bitcast [1000 x i32]* %15 to i8*
  call void @llvm.memset.p0i8.i64(i8* noundef nonnull align 4 dereferenceable(4000) %16, i8 0, i64 4000, i1 false)
  %17 = getelementptr inbounds [1000 x i32], [1000 x i32]* %3, i64 7
  %18 = bitcast [1000 x i32]* %17 to i8*
  call void @llvm.memset.p0i8.i64(i8* noundef nonnull align 4 dereferenceable(4000) %18, i8 0, i64 4000, i1 false)
  %19 = getelementptr inbounds [1000 x i32], [1000 x i32]* %3, i64 8
  %20 = bitcast [1000 x i32]* %19 to i8*
  call void @llvm.memset.p0i8.i64(i8* noundef nonnull align 4 dereferenceable(4000) %20, i8 0, i64 4000, i1 false)
  %21 = getelementptr inbounds [1000 x i32], [1000 x i32]* %3, i64 9
  %22 = bitcast [1000 x i32]* %21 to i8*
  call void @llvm.memset.p0i8.i64(i8* noundef nonnull align 4 dereferenceable(4000) %22, i8 0, i64 4000, i1 false)
  %23 = getelementptr inbounds [1000 x i32], [1000 x i32]* %3, i64 10
  %24 = bitcast [1000 x i32]* %23 to i8*
  call void @llvm.memset.p0i8.i64(i8* noundef nonnull align 4 dereferenceable(4000) %24, i8 0, i64 4000, i1 false)
  %25 = getelementptr inbounds [1000 x i32], [1000 x i32]* %3, i64 11
  %26 = bitcast [1000 x i32]* %25 to i8*
  call void @llvm.memset.p0i8.i64(i8* noundef nonnull align 4 dereferenceable(4000) %26, i8 0, i64 4000, i1 false)
  %27 = getelementptr inbounds [1000 x i32], [1000 x i32]* %3, i64 12
  %28 = bitcast [1000 x i32]* %27 to i8*
  call void @llvm.memset.p0i8.i64(i8* noundef nonnull align 4 dereferenceable(4000) %28, i8 0, i64 4000, i1 false)
  %29 = getelementptr inbounds [1000 x i32], [1000 x i32]* %3, i64 13
  %30 = bitcast [1000 x i32]* %29 to i8*
  call void @llvm.memset.p0i8.i64(i8* noundef nonnull align 4 dereferenceable(4000) %30, i8 0, i64 4000, i1 false)
  %31 = getelementptr inbounds [1000 x i32], [1000 x i32]* %3, i64 14
  %32 = bitcast [1000 x i32]* %31 to i8*
  call void @llvm.memset.p0i8.i64(i8* noundef nonnull align 4 dereferenceable(4000) %32, i8 0, i64 4000, i1 false)
  %33 = getelementptr inbounds [1000 x i32], [1000 x i32]* %3, i64 15
  %34 = bitcast [1000 x i32]* %33 to i8*
  call void @llvm.memset.p0i8.i64(i8* noundef nonnull align 4 dereferenceable(4000) %34, i8 0, i64 4000, i1 false)
  %35 = getelementptr inbounds [1000 x i32], [1000 x i32]* %3, i64 16
  %36 = bitcast [1000 x i32]* %35 to i8*
  call void @llvm.memset.p0i8.i64(i8* noundef nonnull align 4 dereferenceable(4000) %36, i8 0, i64 4000, i1 false)
  %37 = getelementptr inbounds [1000 x i32], [1000 x i32]* %3, i64 17
  %38 = bitcast [1000 x i32]* %37 to i8*
  call void @llvm.memset.p0i8.i64(i8* noundef nonnull align 4 dereferenceable(4000) %38, i8 0, i64 4000, i1 false)
  %39 = getelementptr inbounds [1000 x i32], [1000 x i32]* %3, i64 18
  %40 = bitcast [1000 x i32]* %39 to i8*
  call void @llvm.memset.p0i8.i64(i8* noundef nonnull align 4 dereferenceable(4000) %40, i8 0, i64 4000, i1 false)
  %41 = getelementptr inbounds [1000 x i32], [1000 x i32]* %3, i64 19
  %42 = bitcast [1000 x i32]* %41 to i8*
  call void @llvm.memset.p0i8.i64(i8* noundef nonnull align 4 dereferenceable(4000) %42, i8 0, i64 4000, i1 false)
  %43 = getelementptr inbounds [1000 x i32], [1000 x i32]* %3, i64 20
  %44 = bitcast [1000 x i32]* %43 to i8*
  call void @llvm.memset.p0i8.i64(i8* noundef nonnull align 4 dereferenceable(4000) %44, i8 0, i64 4000, i1 false)
  %45 = getelementptr inbounds [1000 x i32], [1000 x i32]* %3, i64 21
  %46 = bitcast [1000 x i32]* %45 to i8*
  call void @llvm.memset.p0i8.i64(i8* noundef nonnull align 4 dereferenceable(4000) %46, i8 0, i64 4000, i1 false)
  %47 = getelementptr inbounds [1000 x i32], [1000 x i32]* %3, i64 22
  %48 = bitcast [1000 x i32]* %47 to i8*
  call void @llvm.memset.p0i8.i64(i8* noundef nonnull align 4 dereferenceable(4000) %48, i8 0, i64 4000, i1 false)
  %49 = getelementptr inbounds [1000 x i32], [1000 x i32]* %3, i64 23
  %50 = bitcast [1000 x i32]* %49 to i8*
  call void @llvm.memset.p0i8.i64(i8* noundef nonnull align 4 dereferenceable(4000) %50, i8 0, i64 4000, i1 false)
  %51 = getelementptr inbounds [1000 x i32], [1000 x i32]* %3, i64 24
  %52 = bitcast [1000 x i32]* %51 to i8*
  call void @llvm.memset.p0i8.i64(i8* noundef nonnull align 4 dereferenceable(4000) %52, i8 0, i64 4000, i1 false)
  %53 = getelementptr inbounds [1000 x i32], [1000 x i32]* %3, i64 25
  %.not.24 = icmp eq [1000 x i32]* %53, %2
  br i1 %.not.24, label %repeat_loop_next, label %repeat_loop_body

repeat_loop_next:                                 ; preds = %repeat_loop_body
  ret void
}
