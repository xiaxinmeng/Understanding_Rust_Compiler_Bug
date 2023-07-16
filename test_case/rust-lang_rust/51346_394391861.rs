llvm
*** IR Dump After Value Propagation ***
; Function Attrs: uwtable
define i64 @_ZN13nonzero_usize16calculate_layout17h044f84d640378d0bE(i1 zeroext %c) unnamed_addr #0 {
start:
  %spec.select.i = select i1 %c, i64 0, i64 8
  br i1 %c, label %bb2, label %bb4

bb2:                                              ; preds = %start
  unreachable

bb4:                                              ; preds = %start
  %0 = sub i64 8, %spec.select.i
  %1 = insertvalue { i64, i1 } undef, i64 %0, 0
  %2 = insertvalue { i64, i1 } %1, i1 false, 1
  %3 = extractvalue { i64, i1 } %2, 1
  br i1 %3, label %panic, label %bb6, !prof !0

bb6:                                              ; preds = %bb4
  %4 = extractvalue { i64, i1 } %2, 0
  ret i64 %4

panic:                                            ; preds = %bb4
  call ... ; elided
  unreachable
}
*** IR Dump After Simplify the CFG ***
; Function Attrs: uwtable
define i64 @_ZN13nonzero_usize16calculate_layout17h044f84d640378d0bE(i1 zeroext %c) unnamed_addr #0 {
start:
  %spec.select.i = select i1 %c, i64 0, i64 8
  %0 = sub i64 8, %spec.select.i
  %1 = insertvalue { i64, i1 } undef, i64 %0, 0
  %2 = insertvalue { i64, i1 } %1, i1 false, 1
  %3 = extractvalue { i64, i1 } %2, 1
  br i1 %3, label %panic, label %bb6, !prof !0

bb6:                                              ; preds = %start
  %4 = extractvalue { i64, i1 } %2, 0
  ret i64 %4

panic:                                            ; preds = %start
  call ... ; elided
  unreachable
}
