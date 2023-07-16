llvm
define zeroext i1 @_ZN7example3foo17h7a9a6aa0879e3714E(i32) unnamed_addr #0 !dbg !4 {
  call void @llvm.dbg.value(metadata i32 %0, metadata !21, metadata !DIExpression()), !dbg !22
  %1 = icmp ne i32 %0, 0, !dbg !23
  ret i1 %1, !dbg !24
}
