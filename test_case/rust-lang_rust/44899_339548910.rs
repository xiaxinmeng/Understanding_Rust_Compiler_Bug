
*** IR Dump Before Early CSE ***
; Function Attrs: uwtable
define i32 @_ZN3lib16bugging_function17hab1ad2b68c3d5905E(i32) unnamed_addr #2 {
start:
  %1 = icmp ult i32 %0, 3
  call void @llvm.assume(i1 %1)
  %2 = xor i32 %0, -1
  %.off.i = add i32 %2, -1
  %switch.i = icmp ult i32 %.off.i, 2
  br i1 %switch.i, label %_ZN3lib8from_u3217hcbd3743a2503d713E.exit, label %bb3.i

bb3.i:                                            ; preds = %start
  call fastcc void @_ZN3std9panicking11begin_panic17h684cdffa117ea7c2E()
  unreachable

_ZN3lib8from_u3217hcbd3743a2503d713E.exit:        ; preds = %start
  %3 = icmp ult i32 %2, 3
  call void @llvm.assume(i1 %3)
  %4 = or i32 %2, 1
  ret i32 %4
}
