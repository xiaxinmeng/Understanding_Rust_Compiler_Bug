
%9 = load atomic i64, i64* bitcast (i8** getelementptr inbounds (%"core::sync::atomic::AtomicPtr<()>", %"core::sync::atomic::AtomicPtr<()>"* @_ZN6memchr3x867memrchr2FN17h5bc0baf729fbe004E, i64 0, i32 1) to i64*) monotonic, align 8, !noalias !5137
  %10 = inttoptr i64 %9 to { i64, i64 } (i8, [0 x i8]*, i64)*
  %11 = icmp ne i64 %9, 0
  tail call void @llvm.assume(i1 %11), !noalias !5137
