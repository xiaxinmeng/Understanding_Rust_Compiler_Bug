
; Function Attrs: nounwind uwtable
define void @vec_clear(%"alloc::vec::Vec<u32>"* noalias nocapture dereferenceable(24) %x) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %0 = getelementptr inbounds %"alloc::vec::Vec<u32>", %"alloc::vec::Vec<u32>"* %x, i64 0, i32 3
  %1 = getelementptr inbounds %"alloc::vec::Vec<u32>", %"alloc::vec::Vec<u32>"* %x, i64 0, i32 0, i64 0
  %2 = load i64, i64* %1, align 8, !range !0, !alias.scope !1
  %3 = and i64 %2, 3
  %4 = icmp eq i64 %3, 0
  tail call void @llvm.assume(i1 %4) #2, !noalias !10
  store i64 0, i64* %0, align 8, !alias.scope !11, !noalias !12
  ret void
}
