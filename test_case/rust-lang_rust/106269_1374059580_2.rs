llvm
define noundef zeroext i1 @_ZN7example2eq17h1d431ecac5604099E(ptr noalias nocapture noundef readonly align 1 dereferenceable(4) %s1, ptr noalias nocapture noundef readonly align 1 dereferenceable(4) %s2) unnamed_addr #0 !dbg !6 {
  %0 = load i32, ptr %s1, align 1, !dbg !11
  %1 = load i32, ptr %s2, align 1, !dbg !12
  %2 = icmp eq i32 %0, %1, !dbg !13
  ret i1 %2, !dbg !14
}
