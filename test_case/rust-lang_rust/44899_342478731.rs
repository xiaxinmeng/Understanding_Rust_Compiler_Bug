llvm
; Function Attrs: nounwind
declare void @llvm.assume(i1)

; Function Attrs: nounwind uwtable
define i32 @_ZN3ice16bugging_function17hec6c566c29298c46E(i1 zeroext) unnamed_addr #0 {
start:
  %. = select i1 %0, i32 -2, i32 -3
  %.off.i = add i32 %., -1
  %1 = icmp ult i32 %.off.i, 2
  %_0.sroa.0.0.i = zext i1 %1 to i64
  %_0.sroa.3.0.insert.ext.i = zext i32 %. to i64
  %_0.sroa.3.0.insert.shift.i = shl nuw i64 %_0.sroa.3.0.insert.ext.i, 32
  %_0.sroa.0.0.insert.insert.i = or i64 %_0.sroa.0.0.i, %_0.sroa.3.0.insert.shift.i
  %abi_cast.sroa.4.0.extract.shift = lshr i64 %_0.sroa.0.0.insert.insert.i, 32
  %abi_cast.sroa.4.0.extract.trunc = trunc i64 %abi_cast.sroa.4.0.extract.shift to i32
  %2 = and i64 %_0.sroa.0.0.insert.insert.i, 4294967295
  %cond = icmp eq i64 %2, 1
  br i1 %cond, label %bb6, label %bb9

bb6:                                              ; preds = %start
  %3 = icmp ne i32 %abi_cast.sroa.4.0.extract.trunc, 0
  call void @llvm.assume(i1 %3)
  %4 = icmp ult i32 %abi_cast.sroa.4.0.extract.trunc, 3
  call void @llvm.assume(i1 %4)
  br label %bb9

bb9:                                              ; preds = %start, %bb6
  %_0.0 = phi i32 [ %abi_cast.sroa.4.0.extract.trunc, %bb6 ], [ 0, %start ]
  ret i32 %_0.0
}
