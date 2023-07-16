llvm
*** IR Dump After Infer set function attributes ***
; Function Attrs: inlinehint noreturn uwtable
define internal void @_ZN4core4hint21unreachable_unchecked17h5c82d720186d4847E() unnamed_addr #0 {
start:
  unreachable
}

; Function Attrs: uwtable
define internal { i64, i64 } @_ZN13nonzero_usize6repeat17hff27c5667304426aE(i1 zeroext %c) unnamed_addr #1 {
start:
  br i1 %c, label %bb1, label %bb2

bb1:                                              ; preds = %start
  br label %bb3

bb2:                                              ; preds = %start
  br label %bb3

bb3:                                              ; preds = %bb2, %bb1
  %_0.sroa.0.0 = phi i64 [ 0, %bb1 ], [ 1, %bb2 ]
  %0 = insertvalue { i64, i64 } undef, i64 %_0.sroa.0.0, 0
  %1 = insertvalue { i64, i64 } %0, i64 8, 1
  ret { i64, i64 } %1
}

; Function Attrs: uwtable
define i64 @_ZN13nonzero_usize16calculate_layout17h044f84d640378d0bE(i1 zeroext %c) unnamed_addr #1 {
start:
  %0 = call { i64, i64 } @_ZN13nonzero_usize6repeat17hff27c5667304426aE(i1 zeroext %c)
  %.fca.0.extract = extractvalue { i64, i64 } %0, 0
  %.fca.1.extract = extractvalue { i64, i64 } %0, 1
  %switch = icmp ult i64 %.fca.0.extract, 1
  br i1 %switch, label %bb2, label %bb4

bb2:                                              ; preds = %start
  call void @_ZN4core4hint21unreachable_unchecked17h5c82d720186d4847E()
  unreachable

bb4:                                              ; preds = %start
  %1 = call { i64, i1 } @llvm.usub.with.overflow.i64(i64 8, i64 %.fca.1.extract)
  %2 = extractvalue { i64, i1 } %1, 0
  %3 = extractvalue { i64, i1 } %1, 1
  br i1 %3, label %panic, label %bb5, !prof !0

bb5:                                              ; preds = %bb4
  ret i64 %2

panic:                                            ; preds = %bb4
  call void ... ; elided
  unreachable
}

*** IR Dump After Interprocedural Sparse Conditional Constant Propagation ***

; Function Attrs: inlinehint noreturn uwtable
define internal void @_ZN4core4hint21unreachable_unchecked17h5c82d720186d4847E() unnamed_addr #0 {
start:
  unreachable
}

; Function Attrs: uwtable
define internal { i64, i64 } @_ZN13nonzero_usize6repeat17hff27c5667304426aE(i1 zeroext %c) unnamed_addr #1 {
start:
  br i1 %c, label %bb1, label %bb2

bb1:                                              ; preds = %start
  br label %bb3

bb2:                                              ; preds = %start
  br label %bb3

bb3:                                              ; preds = %bb2, %bb1
  %_0.sroa.0.0 = phi i64 [ 0, %bb1 ], [ 1, %bb2 ]
  %0 = insertvalue { i64, i64 } undef, i64 %_0.sroa.0.0, 0
  %1 = insertvalue { i64, i64 } %0, i64 8, 1
  ret { i64, i64 } %1
}

; Function Attrs: uwtable
define i64 @_ZN13nonzero_usize16calculate_layout17h044f84d640378d0bE(i1 zeroext %c) unnamed_addr #1 {
start:
  %0 = call { i64, i64 } @_ZN13nonzero_usize6repeat17hff27c5667304426aE(i1 zeroext %c)
  %.fca.0.extract = extractvalue { i64, i64 } %0, 0
  %switch = icmp ult i64 %.fca.0.extract, 1
  br i1 %switch, label %bb2, label %bb4

bb2:                                              ; preds = %start
  call void @_ZN4core4hint21unreachable_unchecked17h5c82d720186d4847E()
  unreachable

bb4:                                              ; preds = %start
  %1 = call { i64, i1 } @llvm.usub.with.overflow.i64(i64 8, i64 8)
  %2 = extractvalue { i64, i1 } %1, 0
  %3 = extractvalue { i64, i1 } %1, 1
  br i1 %3, label %panic, label %bb5, !prof !0

bb5:                                              ; preds = %bb4
  ret i64 %2

panic:                                            ; preds = %bb4
  call void ... ; elided
  unreachable
}
