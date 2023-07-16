 llvm
define void @_Z4set2Pi(i32* nocapture %x) #0 {
  tail call void @llvm.dbg.value(metadata i32* %x, i64 0, metadata !14, metadata !18), !dbg !26
  tail call void @llvm.dbg.value(metadata i32* %x, i64 0, metadata !27, metadata !18), !dbg !29
  store i32 123, i32* %x, align 4, !dbg !30, !tbaa !21
  ret void, !dbg !31
}
