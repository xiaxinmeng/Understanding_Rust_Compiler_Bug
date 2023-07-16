llvm
define dso_local i8* @_Z12write_stringh(i8 zeroext %0) local_unnamed_addr #0 !dbg !7 {
  call void @llvm.dbg.value(metadata i8 %0, metadata !16, metadata !DIExpression()), !dbg !17
  %2 = xor i8 %0, -128, !dbg !18
  %3 = zext i8 %2 to i64, !dbg !18
  %4 = getelementptr inbounds [256 x i8*], [256 x i8*]* @switch.table._Z12write_stringh, i64 0, i64 %3, !dbg !18
  %5 = load i8*, i8** %4, align 8, !dbg !18
  ret i8* %5, !dbg !18
}
