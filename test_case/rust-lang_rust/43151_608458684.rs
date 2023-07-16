llvm
; ModuleID = 'foo.3a1fbbbh-cgu.0'
source_filename = "foo.3a1fbbbh-cgu.0"
target datalayout = "e-m:w-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-gnu"

%A = type {}
%"unwind::libunwind::EXCEPTION_RECORD" = type { [0 x i8] }
%"unwind::libunwind::CONTEXT" = type { [0 x i8] }
%"unwind::libunwind::DISPATCHER_CONTEXT" = type { [0 x i8] }

; <foo::A as core::ops::drop::Drop>::drop
; Function Attrs: norecurse nounwind readnone uwtable
define void @"_ZN48_$LT$foo..A$u20$as$u20$core..ops..drop..Drop$GT$4drop17h095156138f26c8b3E"(%A* nocapture nonnull align 1 %self) unnamed_addr #0 {
start:
  ret void
}

; foo::foo
; Function Attrs: uwtable
define void @_ZN3foo3foo17he8a114b84f989db4E() unnamed_addr #1 personality i32 (%"unwind::libunwind::EXCEPTION_RECORD"*, i8*, %"unwind::libunwind::CONTEXT"*, %"unwind::libunwind::DISPATCHER_CONTEXT"*)* @rust_eh_personality {
start:
; call bar::bar
  tail call void @_ZN3bar3bar17h6d6497eda9d60abfE()
  ret void
}

; Function Attrs: nounwind uwtable
declare i32 @rust_eh_personality(%"unwind::libunwind::EXCEPTION_RECORD"*, i8*, %"unwind::libunwind::CONTEXT"*, %"unwind::libunwind::DISPATCHER_CONTEXT"*) unnamed_addr #2

; bar::bar
; Function Attrs: uwtable
declare void @_ZN3bar3bar17h6d6497eda9d60abfE() unnamed_addr #1

attributes #0 = { norecurse nounwind readnone uwtable "target-cpu"="x86-64" }
attributes #1 = { uwtable "target-cpu"="x86-64" }
attributes #2 = { nounwind uwtable "target-cpu"="x86-64" }

!llvm.module.flags = !{!0}

!0 = !{i32 7, !"PIC Level", i32 2}
