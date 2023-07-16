llvm
define void @_ZN7example12min_array_ok17h64ef6889d5bfe023E(ptr sret([1 x i128]) %0) unnamed_addr #0 !dbg !6 {
start:
  %1 = getelementptr inbounds [1 x i128], ptr %0, i64 0, i64 0, !dbg !11
  store i128 -170141183460469231731687303715884105728, ptr %1, align 8, !dbg !11
  ret void, !dbg !12
}

define void @_ZN7example13min_array_nok17h5f347fb6a42ac96fE(ptr sret([1 x i128]) %0) unnamed_addr #0 !dbg !13 {
start:
  %1 = getelementptr inbounds [1 x i128], ptr %0, i64 0, i64 0, !dbg !14
  call void @llvm.memset.p0.i64(ptr align 8 %1, i8 0, i64 16, i1 false), !dbg !14
  ret void, !dbg !15
}
