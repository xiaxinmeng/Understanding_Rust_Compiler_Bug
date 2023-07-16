 llvm
; ModuleID = 'mintest.0.rs'
target datalayout = "e-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@const10 = internal unnamed_addr constant i32 0
@const14 = internal unnamed_addr constant i32 128

; Function Attrs: uwtable
define internal void @_ZN4main20h4fb6a247161ac946eaaE() unnamed_addr #0 {
entry-block:
  %byte = alloca i32
  %port = alloca i32
  store i32 0, i32* %byte
  store i32 128, i32* %port
  %0 = load i32* %byte
  %1 = load i32* %port
  call void asm sideeffect "out %al, %dx", "a,d,~{dirflag},~{fpsr},~{flags}"(i32 %0, i32 %1), !srcloc !0
  ret void
}

define i64 @main(i64, i8**) unnamed_addr #1 {
top:
  %2 = call i64 @_ZN2rt10lang_start20hb1b8686306685c6ftRLE(i8* bitcast (void ()* @_ZN4main20h4fb6a247161ac946eaaE to i8*), i64 %0, i8** %1)
  ret i64 %2
}

declare i64 @_ZN2rt10lang_start20hb1b8686306685c6ftRLE(i8*, i64, i8**) unnamed_addr #1

attributes #0 = { uwtable "split-stack" }
attributes #1 = { "split-stack" }

!0 = !{i32 1}
