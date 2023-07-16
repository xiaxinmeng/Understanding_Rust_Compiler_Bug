llvm
@_ZN10playground1X17h7a5538bb38d6a168E = internal unnamed_addr constant <{ [32 x i8] }> <{ [32 x i8] c"\0A\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00\04\00\00\00\00\00\00\00\03\00\00\00\00\00\00\00" }>, align 8

; playground::foo
; Function Attrs: nounwind nonlazybind uwtable
define i64 @_ZN10playground3foo17hc0370d8e3b68fd0eE(i8 %y) unnamed_addr #0 {
start:
  %0 = icmp ult i8 %y, 4
  tail call void @llvm.assume(i1 %0)
  %_3 = zext i8 %y to i64
  %1 = getelementptr inbounds [4 x i64], [4 x i64]* bitcast (<{ [32 x i8] }>* @_ZN10playground1X17h7a5538bb38d6a168E to [4 x i64]*), i64 0, i64 %_3
  %2 = load i64, i64* %1, align 8
  ret i64 %2
}

; Function Attrs: nounwind
declare void @llvm.assume(i1) #1

attributes #0 = { nounwind nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #1 = { nounwind }

!llvm.module.flags = !{!0, !1}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
