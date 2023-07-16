
; playground::map_on_the_elems
; Function Attrs: uwtable
define void @_ZN10playground16map_on_the_elems17h382723b074a63bf8E(%"alloc::vec::Vec<u64>"* noalias nocapture readonly dereferenceable(24), void (i64)* nocapture) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %.idx = getelementptr %"alloc::vec::Vec<u64>", %"alloc::vec::Vec<u64>"* %0, i64 0, i32 0, i32 0, i32 0, i32 0
  %.idx.val = load i64*, i64** %.idx, align 8, !alias.scope !0
  %.idx3 = getelementptr %"alloc::vec::Vec<u64>", %"alloc::vec::Vec<u64>"* %0, i64 0, i32 2
  %.idx3.val = load i64, i64* %.idx3, align 8, !alias.scope !5
  %2 = icmp eq i64 %.idx3.val, 0
  br i1 %2, label %bb6, label %bb8.preheader

bb8.preheader:                                    ; preds = %start
  br label %bb8

bb6.loopexit:                                     ; preds = %bb8
  br label %bb6

bb6:                                              ; preds = %bb6.loopexit, %start
  ret void

bb8:                                              ; preds = %bb8.preheader, %bb8
  %iter.sroa.0.012 = phi i64 [ %3, %bb8 ], [ 0, %bb8.preheader ]
  %3 = add nuw i64 %iter.sroa.0.012, 1
  %4 = getelementptr inbounds i64, i64* %.idx.val, i64 %iter.sroa.0.012
  %5 = load i64, i64* %4, align 8
  tail call void %1(i64 %5)
  %6 = icmp ult i64 %3, %.idx3.val
  br i1 %6, label %bb8, label %bb6.loopexit
}
