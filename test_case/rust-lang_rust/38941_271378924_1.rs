llvm
; ModuleID = 'noalias.cgu-0.rs'
source_filename = "noalias.cgu-0.rs"
target datalayout = "e-m:e-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

; Function Attrs: uwtable
define void @_ZN7noalias3foo17h1c56d659e6e1ab4bE(i32* dereferenceable(4), i32* dereferenceable(4), i32*) unnamed_addr #0 {
entry-block:
  %_0 = alloca {}
  br label %start

start:                                            ; preds = %entry-block
  %3 = load i32, i32* %2
  store i32 %3, i32* %0
  %4 = load i32, i32* %2
  store i32 %4, i32* %1
  ret void
}

attributes #0 = { uwtable }
