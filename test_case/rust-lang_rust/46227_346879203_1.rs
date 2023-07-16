
; playground::map_on_the_elems
; Function Attrs: uwtable
define void @_ZN10playground16map_on_the_elems17h382723b074a63bf8E(%"alloc::vec::Vec<u64>"* noalias nocapture readonly dereferenceable(24), void (i64)* nocapture) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %.idx = getelementptr %"alloc::vec::Vec<u64>", %"alloc::vec::Vec<u64>"* %0, i64 0, i32 2
  %.idx.val = load i64, i64* %.idx, align 8
  %2 = icmp eq i64 %.idx.val, 0
  br i1 %2, label %bb5, label %bb2.i.lr.ph

bb2.i.lr.ph:                                      ; preds = %start
  %.idx4 = getelementptr %"alloc::vec::Vec<u64>", %"alloc::vec::Vec<u64>"* %0, i64 0, i32 0, i32 0, i32 0, i32 0
  %.idx4.val = load i64*, i64** %.idx4, align 8
  br label %bb6

bb5.loopexit:                                     ; preds = %"_ZN81_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..ops..index..Index$LT$usize$GT$$GT$5index17h7a894b1ad960f72cE.exit"
  br label %bb5

bb5:                                              ; preds = %bb5.loopexit, %start
  ret void

bb6:                                              ; preds = %"_ZN81_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..ops..index..Index$LT$usize$GT$$GT$5index17h7a894b1ad960f72cE.exit", %bb2.i.lr.ph
  %iter.sroa.0.014 = phi i64 [ 0, %bb2.i.lr.ph ], [ %3, %"_ZN81_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..ops..index..Index$LT$usize$GT$$GT$5index17h7a894b1ad960f72cE.exit" ]
  %3 = add nuw i64 %iter.sroa.0.014, 1
  %4 = icmp ugt i64 %.idx.val, %iter.sroa.0.014
  br i1 %4, label %"_ZN81_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..ops..index..Index$LT$usize$GT$$GT$5index17h7a894b1ad960f72cE.exit", label %panic.i, !prof !0

panic.i:                                          ; preds = %bb6
; call core::panicking::panic_bounds_check
  tail call void @_ZN4core9panicking18panic_bounds_check17h00738207f5476c9aE({ %str_slice, [0 x i8], i32, [0 x i8], i32, [0 x i8] }* noalias readonly dereferenceable(24) bitcast ({ %str_slice, i32, i32 }* @panic_bounds_check_loc.1 to { %str_slice, [0 x i8], i32, [0 x i8], i32, [0 x i8] }*), i64 %iter.sroa.0.014, i64 %.idx.val)
  unreachable

"_ZN81_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..ops..index..Index$LT$usize$GT$$GT$5index17h7a894b1ad960f72cE.exit": ; preds = %bb6
  %5 = getelementptr inbounds i64, i64* %.idx4.val, i64 %iter.sroa.0.014
  %6 = load i64, i64* %5, align 8
  tail call void %1(i64 %6)
  %7 = icmp ult i64 %3, %.idx.val
  br i1 %7, label %bb6, label %bb5.loopexit
}
