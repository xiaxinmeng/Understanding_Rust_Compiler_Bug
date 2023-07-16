llvm
; ModuleID = 'playground0-3a317160927575f9dc8b26750a523322.rs'
source_filename = "playground0-3a317160927575f9dc8b26750a523322.rs"
target datalayout = "e-m:e-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%"unwind::libunwind::_Unwind_Exception" = type { [0 x i64], i64, [0 x i64], void (i32, %"unwind::libunwind::_Unwind_Exception"*)*, [0 x i64], [6 x i64], [0 x i64] }
%"unwind::libunwind::_Unwind_Context" = type { [0 x i8] }

; Function Attrs: uwtable
define void @foo(i64** noalias nocapture readonly dereferenceable(8) %t) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %t.val = load i64*, i64** %t, align 8
  %.val.i.i.i = load i64, i64* %t.val, align 8
  %0 = tail call { i64, i1 } @llvm.uadd.with.overflow.i64(i64 %.val.i.i.i, i64 1) #5
  %1 = extractvalue { i64, i1 } %0, 1
  br i1 %1, label %bb2.i.i.i, label %"_ZN61_$LT$alloc..rc..Rc$LT$T$GT$$u20$as$u20$core..clone..Clone$GT$5clone17h127f785fcc974269E.exit"

bb2.i.i.i:                                        ; preds = %start
  tail call void @llvm.trap() #5
  unreachable

"_ZN61_$LT$alloc..rc..Rc$LT$T$GT$$u20$as$u20$core..clone..Clone$GT$5clone17h127f785fcc974269E.exit": ; preds = %start
  %2 = extractvalue { i64, i1 } %0, 0
  %3 = add i64 %2, -1
  store i64 %3, i64* %t.val, align 1
  %4 = icmp eq i64 %3, 0
  br i1 %4, label %bb4.i.i.i, label %_ZN4core3mem4drop17h222f528d1fa068ceE.exit

bb4.i.i.i:                                        ; preds = %"_ZN61_$LT$alloc..rc..Rc$LT$T$GT$$u20$as$u20$core..clone..Clone$GT$5clone17h127f785fcc974269E.exit"
  %5 = getelementptr inbounds i64, i64* %t.val, i64 1
  %.val.i.i5.i.i.i = load i64, i64* %5, align 8
  %6 = add i64 %.val.i.i5.i.i.i, -1
  store i64 %6, i64* %5, align 1
  %7 = icmp eq i64 %6, 0
  br i1 %7, label %bb9.i.i.i, label %_ZN4core3mem4drop17h222f528d1fa068ceE.exit

bb9.i.i.i:                                        ; preds = %bb4.i.i.i
  %8 = bitcast i64* %t.val to i8*
  tail call void @__rust_dealloc(i8* nonnull %8, i64 24, i64 8) #5
  br label %_ZN4core3mem4drop17h222f528d1fa068ceE.exit

_ZN4core3mem4drop17h222f528d1fa068ceE.exit:       ; preds = %"_ZN61_$LT$alloc..rc..Rc$LT$T$GT$$u20$as$u20$core..clone..Clone$GT$5clone17h127f785fcc974269E.exit", %bb4.i.i.i, %bb9.i.i.i
  ret void
}

declare i32 @rust_eh_personality(i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*) unnamed_addr #1

; Function Attrs: nounwind readnone speculatable
declare { i64, i1 } @llvm.uadd.with.overflow.i64(i64, i64) #2

; Function Attrs: noreturn nounwind
declare void @llvm.trap() #3

; Function Attrs: nounwind
declare void @__rust_dealloc(i8*, i64, i64) unnamed_addr #4

attributes #0 = { uwtable "probe-stack"="__rust_probestack" }
attributes #1 = { "probe-stack"="__rust_probestack" }
attributes #2 = { nounwind readnone speculatable }
attributes #3 = { noreturn nounwind }
attributes #4 = { nounwind "probe-stack"="__rust_probestack" }
attributes #5 = { nounwind }
