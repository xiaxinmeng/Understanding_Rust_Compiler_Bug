llvm
; playground::f
; Function Attrs: norecurse nounwind nonlazybind readnone uwtable willreturn
define zeroext i1 @_ZN10playground1f17h98eba4f4ca80f23cE(i32 %0, i32 %1) unnamed_addr #0 {
start:
  %2 = icmp ne i32 %0, 0
  %3 = icmp ne i32 %1, 0
  %4 = xor i1 %2, %3
  br i1 %4, label %bb2.i.i, label %bb1.i.i

bb2.i.i:                                          ; preds = %start
  %5 = xor i1 %2, true
  %_3.i.i.i.i = and i1 %3, %5
  %.0.i.i.i.i = select i1 %_3.i.i.i.i, i8 -1, i8 1
  br label %_ZN4core3cmp10PartialOrd2lt17h869f0d8a39125a29E.exit

bb1.i.i:                                          ; preds = %start
  %.not.i.i = icmp eq i32 %0, 0
  %.not5.i.i = icmp eq i32 %1, 0
  %or.cond.i.i = or i1 %.not.i.i, %.not5.i.i
  br i1 %or.cond.i.i, label %_ZN4core3cmp10PartialOrd2lt17h869f0d8a39125a29E.exit, label %bb5.i.i

bb5.i.i:                                          ; preds = %bb1.i.i
  %_3.i.i.i.i.i = icmp ult i32 %0, %1
  %_6.i.i.i.i.i = icmp ne i32 %0, %1
  %spec.select.i.i.i.i.i = zext i1 %_6.i.i.i.i.i to i8
  %.0.i.i.i.i.i = select i1 %_3.i.i.i.i.i, i8 -1, i8 %spec.select.i.i.i.i.i
  br label %_ZN4core3cmp10PartialOrd2lt17h869f0d8a39125a29E.exit

_ZN4core3cmp10PartialOrd2lt17h869f0d8a39125a29E.exit: ; preds = %bb2.i.i, %bb1.i.i, %bb5.i.i
  %.2.i.i = phi i8 [ %.0.i.i.i.i, %bb2.i.i ], [ 0, %bb1.i.i ], [ %.0.i.i.i.i.i, %bb5.i.i ]
  %cond.i = icmp eq i8 %.2.i.i, -1
  ret i1 %cond.i
}
