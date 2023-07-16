
; playground::map_on_the_elems
; Function Attrs: uwtable
define void @_ZN10playground16map_on_the_elems17h382723b074a63bf8E(%"alloc::vec::Vec<u64>"* noalias nocapture readonly dereferenceable(24), void (i64)* nocapture) unnamed_addr #0 {
start:
  %.idx = getelementptr %"alloc::vec::Vec<u64>", %"alloc::vec::Vec<u64>"* %0, i64 0, i32 2
  %.idx.val = load i64, i64* %.idx, align 8
  %2 = icmp eq i64 %.idx.val, 0
  br i1 %2, label %bb3, label %bb4.lr.ph

bb4.lr.ph:                                        ; preds = %start
  %.idx4 = getelementptr %"alloc::vec::Vec<u64>", %"alloc::vec::Vec<u64>"* %0, i64 0, i32 0, i32 0, i32 0, i32 0
  %.idx4.val = load i64*, i64** %.idx4, align 8
  br label %bb4

bb3.loopexit:                                     ; preds = %_ZN10playground5index17hb2f22227db464aacE.exit
  br label %bb3

bb3:                                              ; preds = %bb3.loopexit, %start
  ret void

bb4:                                              ; preds = %bb4.lr.ph, %_ZN10playground5index17hb2f22227db464aacE.exit
  %i.07 = phi i64 [ 0, %bb4.lr.ph ], [ %6, %_ZN10playground5index17hb2f22227db464aacE.exit ]
  %3 = icmp ult i64 %i.07, %.idx.val
  br i1 %3, label %_ZN10playground5index17hb2f22227db464aacE.exit, label %panic.i, !prof !0

panic.i:                                          ; preds = %bb4
; call core::panicking::panic_bounds_check
  tail call void @_ZN4core9panicking18panic_bounds_check17h00738207f5476c9aE({ %str_slice, [0 x i8], i32, [0 x i8], i32, [0 x i8] }* noalias readonly dereferenceable(24) bitcast ({ %str_slice, i32, i32 }* @panic_bounds_check_loc.1 to { %str_slice, [0 x i8], i32, [0 x i8], i32, [0 x i8] }*), i64 %i.07, i64 %.idx.val), !noalias !1
  unreachable

_ZN10playground5index17hb2f22227db464aacE.exit:   ; preds = %bb4
  %4 = getelementptr inbounds i64, i64* %.idx4.val, i64 %i.07
  %5 = load i64, i64* %4, align 8, !alias.scope !1
  tail call void %1(i64 %5)
  %6 = add nuw i64 %i.07, 1
  %7 = icmp ugt i64 %.idx.val, %6
  br i1 %7, label %bb4, label %bb3.loopexit
}
