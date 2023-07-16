ll
; ModuleID = 'foo.cgu-0.rs'
source_filename = "foo.cgu-0.rs"
target datalayout = "e-m:e-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%Foo = type { i8, i8 }
%"unwind::libunwind::_Unwind_Exception" = type { i64, void (i32, %"unwind::libunwind::_Unwind_Exception"*)*, [6 x i64] }
%"unwind::libunwind::_Unwind_Context" = type {}

; Function Attrs: nounwind uwtable
define void @drop(%Foo* noalias nonnull, i64) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh
_personality {
entry-block:
  %2 = add i64 %1, 1
  %3 = icmp eq i64 %2, 0
  br i1 %3, label %_ZN4drop17hc3530aa457fb35acE.exit, label %cond.i

cond.i:                                           ; preds = %entry-block
  %4 = getelementptr inbounds %Foo, %Foo* %0, i64 0, i32 0
  tail call void @__rust_deallocate(i8* %4, i64 %2, i64 1) #1
  br label %_ZN4drop17hc3530aa457fb35acE.exit

_ZN4drop17hc3530aa457fb35acE.exit:                ; preds = %entry-block, %cond.i
  ret void
}

; Function Attrs: nounwind
declare void @__rust_deallocate(i8*, i64, i64) unnamed_addr #1

; Function Attrs: nounwind
declare i32 @rust_eh_personality(i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*) unnamed_addr #1

