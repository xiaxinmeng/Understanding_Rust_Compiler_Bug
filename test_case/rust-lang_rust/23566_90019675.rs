 llvm
define void @_ZN4set120h703cb75c1e01d663eaaE(i32* noalias nocapture dereferenceable(4)) unnamed_addr #0 {
entry-block:
  tail call void @llvm.dbg.value(metadata i32* %0, i64 0, metadata !14, metadata !20), !dbg !21
  store i32 123, i32* %0, align 4, !dbg !22
  ret void, !dbg !21
}

define void @_ZN4set220h34cbfe792e780cc1paaE(i32* noalias nocapture dereferenceable(4)) unnamed_addr #0 {
  tail call void @_ZN4set120h703cb75c1e01d663eaaE(i32* %0)
  ret void
}
