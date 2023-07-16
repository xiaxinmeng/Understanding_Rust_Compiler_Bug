
define i32 @test({ i64, i64 }) unnamed_addr #0 !dbg !176 {
entry-block:
  %s = alloca %S
  %1 = bitcast %S* %s to { i64, i64 }*
  store { i64, i64 } %0, { i64, i64 }* %1, align 4
  call void @llvm.dbg.declare(metadata %S* %s, metadata !202, metadata !203), !dbg !204
  %2 = getelementptr inbounds %S, %S* %s, i32 0, i32 2
  %3 = load i32, i32* %2, !dbg !205
  ret i32 %3, !dbg !207
}
