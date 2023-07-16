llvm
; playground::iter_too
; Function Attrs: nonlazybind uwtable
define void @_ZN10playground8iter_too17he8dda29a6929fd5aE([0 x i32]* noalias nonnull readonly align 4 %v.0, i64 %v.1) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %0 = getelementptr [0 x i32], [0 x i32]* %v.0, i64 0, i64 0
  %1 = getelementptr inbounds [0 x i32], [0 x i32]* %v.0, i64 0, i64 %v.1
  br label %bb4

bb4:                                              ; preds = %bb8, %start
  %iter.sroa.0.0 = phi i32* [ %0, %start ], [ %2, %bb8 ]
  %iter.sroa.7.0 = phi i64 [ 0, %start ], [ %_11.0.i, %bb8 ]
  %_12.i.i = icmp eq i32* %iter.sroa.0.0, %1
  br i1 %_12.i.i, label %bb6, label %bb8

bb6:                                              ; preds = %bb4
  ret void

bb8:                                              ; preds = %bb4
  %_11.0.i = add nuw nsw i64 %iter.sroa.7.0, 1
  %2 = getelementptr inbounds i32, i32* %iter.sroa.0.0, i64 1
  %_16.not = icmp ugt i64 %iter.sroa.7.0, %v.1
  br i1 %_16.not, label %bb9, label %bb4

bb9:                                              ; preds = %bb8
; call core::panicking::panic
  tail call void @_ZN4core9panicking5panic17h0ba7146865b2f9d6E([0 x i8]* noalias nonnull readonly align 1 bitcast (<{ [30 x i8] }>* @alloc19 to [0 x i8]*), i64 30, %"core::panic::location::Location"* noalias readonly align 8 dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @alloc21 to %"core::panic::location::Location"*)) #2
  unreachable
}
