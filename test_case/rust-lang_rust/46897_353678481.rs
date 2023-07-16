llvm
; Function Attrs: inlinehint nounwind uwtable
define internal fastcc void @"_ZN4core4iter5range93_$LT$impl$u20$core..iter..iterator..Iterator$u20$for$u20$core..ops..range..Range$LT$A$GT$$GT$4next17h9ca7473ffcbaaeeeE"(%"core::option::Option<usize>"* noalias nocapture sret dereferenceable(16), { i64, i64 }* nocapture dereferenceable(16) %self) unnamed_addr #3 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %1 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %self, i64 0, i32 0
  %2 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %self, i64 0, i32 1
  %3 = load i64, i64* %1, align 8, !alias.scope !1, !noalias !4
  %4 = load i64, i64* %2, align 8, !alias.scope !4, !noalias !1
  %5 = icmp ult i64 %3, %4
  br i1 %5, label %bb3, label %bb12

bb3:                                              ; preds = %start
  %6 = tail call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %3, i64 1) #6
  %7 = extractvalue { i64, i1 } %6, 1
  br i1 %7, label %bb12, label %bb7

bb7:                                              ; preds = %bb3
  %8 = extractvalue { i64, i1 } %6, 0
  store i64 %8, i64* %1, align 1
  %9 = getelementptr inbounds %"core::option::Option<usize>", %"core::option::Option<usize>"* %0, i64 0, i32 0, i64 0
  store i64 1, i64* %9, align 8
  br label %bb12

bb12:                                             ; preds = %start, %bb7, %bb3
  %.sink16 = phi i64 [ 1, %bb7 ], [ 0, %bb3 ], [ 0, %start ]
  %.sink14 = phi i64 [ %3, %bb7 ], [ 0, %bb3 ], [ 0, %start ]
  ; WHAT?
  %10 = getelementptr inbounds %"core::option::Option<usize>", %"core::option::Option<usize>"* %0, i64 0, i32 0, i64 %.sink16
  store i64 %.sink14, i64* %10, align 8
  ret void
}
