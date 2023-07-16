
define i32 @_ZN4test3foo17hbf0270950fc5c651E(i32 %num) unnamed_addr #0 {
start:
  %_3 = icmp sge i32 %num, 100
  br i1 %_3, label %bb2, label %bb1

bb1:                                              ; preds = %start
  br label %bb3

bb2:                                              ; preds = %start
  %_5 = icmp slt i32 %num, 200
  %0 = zext i1 %_5 to i8
  br label %bb3

bb3:                                              ; preds = %bb2, %bb1
  %_2.0 = phi i8 [ %0, %bb2 ], [ 0, %bb1 ]
  %1 = trunc i8 %_2.0 to i1
  br i1 %1, label %bb4, label %bb5

bb4:                                              ; preds = %bb3
  %_9 = icmp eq i32 %num, -2147483648
  br i1 false, label %panic, label %bb6, !prof !2

bb5:                                              ; preds = %bb3
  br label %bb7

bb6:                                              ; preds = %bb4
  %2 = sdiv i32 %num, 100
  br label %bb7

bb7:                                              ; preds = %bb6, %bb5
  %.0 = phi i32 [ %2, %bb6 ], [ 1, %bb5 ]
  ret i32 %.0

panic:                                            ; preds = %bb4
  call void @_ZN4core9panicking5panic17h3de4db67bd397eb3E([0 x i8]* noalias nonnull readonly align 1 bitcast ([31 x i8]* @str.0 to [0 x i8]*), i64 31, %"std::panic::Location"* noalias readonly align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @alloc2 to %"std::panic::Location"*))
  unreachable
}
