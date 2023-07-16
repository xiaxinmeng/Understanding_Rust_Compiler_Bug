
; Function Attrs: inlinehint uwtable
define internal i64 @_ZN4core3mem11size_of_val17hd726290847184a20E(%"5.std::rc::RcBox<std::cell::RefCell<[u8]>>"* nonnull, i64) unnamed_addr #1 {
entry-block:
  %tmp_ret = alloca i64
  br label %start

start:                                            ; preds = %entry-block
  %2 = mul i64 %1, 1
  %3 = add i64 0, %2
  %4 = add i64 %3, 0
  %5 = add i64 8, %4
  %6 = add i64 %5, 7
  %7 = and i64 %6, -8
  %8 = add i64 24, %7
  %9 = add i64 %8, 7
  %10 = and i64 %9, -8
  store i64 %10, i64* %tmp_ret
  %11 = load i64, i64* %tmp_ret
  br label %bb1

bb1:                                              ; preds = %start
  ret i64 %11
}

