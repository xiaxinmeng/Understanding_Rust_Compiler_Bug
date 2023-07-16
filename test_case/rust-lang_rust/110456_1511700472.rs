llvm
define noundef i32 @_ZN7example1f17h05bced40beed593aE(i32 noundef %0, i32 returned %1) unnamed_addr #0 !dbg !6 {
start:
  %2 = icmp ne i32 %0, 1, !dbg !11
  tail call void @llvm.assume(i1 %2), !dbg !23
  %switch = icmp eq i32 %0, 0, !dbg !24
  br i1 %switch, label %bb4, label %bb6, !dbg !24

bb4:                                              ; preds = %start
  tail call void @_ZN4core9panicking5panic17h9caa3ddd0cccb1d2E(ptr noalias noundef nonnull readonly align 1 @alloc_5f55955de67e57c79064b537689facea, i64 noundef 43, ptr noalias noundef nonnull readonly align 8 dereferenceable(24) @alloc_b5761c36d4207a96188a90095c0f36d6) #3, !dbg !27
  unreachable, !dbg !27

bb6:                                              ; preds = %start
  ret i32 %1, !dbg !28
}
