llvm
; ModuleID = 'llvm-link'
source_filename = "llvm-link"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@g = unnamed_addr alias { [0 x i64]*, i64 } (), bitcast ({ [0 x i8]*, i64 } ()* @f to { [0 x i64]*, i64 } ()*)

define { [0 x i8]*, i64 } @f() unnamed_addr {
start:
  ret { [0 x i8]*, i64 } undef
}

define void @h() unnamed_addr {
start:
  %0 = tail call { [0 x i8]*, i64 } @f()
  %1 = tail call { [0 x i64]*, i64 } @g()
  ret void
}
