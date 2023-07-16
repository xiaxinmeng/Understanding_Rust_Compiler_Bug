
define i64 @_ZN10playground6access17h378ae4778586ad8dE([12 x i64]* noalias nocapture readonly align 8 dereferenceable(96) %array, i8 %exc) unnamed_addr #0 {
start:
  %0 = icmp ult i8 %exc, 11
  tail call void @llvm.assume(i1 %0)
  %_4 = add nsw i8 %exc, -4
  %_3 = zext i8 %_4 to i64
  %_8 = icmp ult i8 %_4, 12
  br i1 %_8, label %bb1, label %panic, !prof !2

bb1:                                              ; preds = %start
  %1 = getelementptr inbounds [12 x i64], [12 x i64]* %array, i64 0, i64 %_3
  %2 = load i64, i64* %1, align 8
  ret i64 %2

panic:                                            ; preds = %start
; call core::panicking::panic_bounds_check
  tail call void @_ZN4core9panicking18panic_bounds_check17h09b793daa6d169ffE(%"core::panic::Location"* noalias readonly align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @anon.2bcef93dabe22e774fed7ccaab37f6cf.1 to %"core::panic::Location"*), i64 %_3, i64 12)
  unreachable
}
