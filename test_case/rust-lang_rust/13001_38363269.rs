 llvm
; ModuleID = 'issue-13001.rs'
target datalayout = "e-p:64:64:64-i1:8:8-i8:8:8-i16:16:16-i32:32:32-i64:64:64-f32:32:32-f64:64:64-v64:64:64-v128:128:128-a0:0:64-s0:64:64-f80:128:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

declare void @other()

define i64 @without(i64 (i64*)*) unnamed_addr #4 {
entry-block:
  %1 = alloca i64
  %2 = call i64 %0(i64* %1)
  store i64 8, i64* %1
  call void @other()
  ret i64 %2
}

define i64 @with(i64 (i64*)*) unnamed_addr #4 {
entry-block:
  %1 = alloca i64
  %2 = call i64 %0(i64* nocapture %1)
  store i64 8, i64* %1
  call void @other()
  ret i64 %2
}
