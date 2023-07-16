
define i64 @_ZN10playground3new17hc1cbbc83e9256887E(i64 %c) unnamed_addr #2 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %0 = tail call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %c, i64 8) #8
  %1 = extractvalue { i64, i1 } %0, 1
  %2 = extractvalue { i64, i1 } %0, 0
  %spec.select.i.i = select i1 %1, i64 0, i64 8
  br i1 %1, label %bb4.i1, label %bb4.i

bb4.i:                                            ; preds = %start
  %3 = tail call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %c, i64 16) #8
  %4 = extractvalue { i64, i1 } %3, 1
  %5 = extractvalue { i64, i1 } %3, 0
  %spec.select.i16.i = select i1 %4, i64 0, i64 8
  br i1 %4, label %bb4.i1, label %bb11.i

bb11.i:                                           ; preds = %bb4.i
  %6 = icmp ult i64 %spec.select.i16.i, %spec.select.i.i
  %_0.0.sroa.speculated.i.i.i.i = select i1 %6, i64 %spec.select.i.i, i64 %spec.select.i16.i
  %7 = add i64 %2, -1
  %8 = add i64 %7, %spec.select.i16.i
  %9 = sub nsw i64 0, %spec.select.i16.i
  %10 = and i64 %8, %9
  %11 = sub i64 %10, %2
  %12 = tail call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %2, i64 %11) #8
  %13 = extractvalue { i64, i1 } %12, 0
  %14 = extractvalue { i64, i1 } %12, 1
  br i1 %14, label %bb4.i1, label %bb11.i.i

bb11.i.i:                                         ; preds = %bb11.i
  %15 = tail call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %13, i64 %5) #8
  %16 = extractvalue { i64, i1 } %15, 1
  br i1 %16, label %bb4.i1, label %bb21.i.i

bb21.i.i:                                         ; preds = %bb11.i.i
  %17 = extractvalue { i64, i1 } %15, 0
  %18 = add nuw nsw i64 %_0.0.sroa.speculated.i.i.i.i, 15
  %19 = and i64 %18, %_0.0.sroa.speculated.i.i.i.i
  %20 = icmp ne i64 %19, 0
  %21 = icmp eq i64 %_0.0.sroa.speculated.i.i.i.i, 0
  %or.cond.i.not.i.i.i = or i1 %21, %20
  %22 = sub nsw i64 0, %_0.0.sroa.speculated.i.i.i.i
  %23 = icmp ugt i64 %17, %22
  %24 = or i1 %or.cond.i.not.i.i.i, %23
  br i1 %24, label %bb4.i1, label %"_ZN47_$LT$core..result..Result$LT$T$C$$u20$E$GT$$GT$6unwrap17h2368a06f0a7bf993E.exit"

bb4.i1:                                           ; preds = %bb21.i.i, %bb4.i, %bb11.i.i, %bb11.i, %start
; call core::result::unwrap_failed
  tail call fastcc void @_ZN4core6result13unwrap_failed17hde364ddca953ee8aE(), !noalias !6
  unreachable

"_ZN47_$LT$core..result..Result$LT$T$C$$u20$E$GT$$GT$6unwrap17h2368a06f0a7bf993E.exit": ; preds = %bb21.i.i
  ret i64 %13
}
