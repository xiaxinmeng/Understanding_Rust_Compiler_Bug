
; ModuleID = 'test.rs'
target datalayout = "e-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

define i64 @main(i64, i8**) unnamed_addr {
top:
  %__adjust.i = alloca { void (i8*)*, i8* }, align 8
  %2 = bitcast { void (i8*)*, i8* }* %__adjust.i to i8*
  call void @llvm.lifetime.start(i64 16, i8* %2)
  %3 = getelementptr inbounds { void (i8*)*, i8* }* %__adjust.i, i64 0, i32 0
  store void (i8*)* @_ZN4main14as_closure.983E, void (i8*)** %3, align 8
  %4 = getelementptr inbounds { void (i8*)*, i8* }* %__adjust.i, i64 0, i32 1
  store i8* null, i8** %4, align 8
  %5 = call i64 @_ZN5start20h52ed3dec7308a2ebxzd9v0.11.preE(i64 %0, i8** %1, { void (i8*)*, i8* }* nocapture %__adjust.i)
  call void @llvm.lifetime.end(i64 16, i8* %2)
  ret i64 %5
}

declare i64 @_ZN5start20h52ed3dec7308a2ebxzd9v0.11.preE(i64, i8**, { void (i8*)*, i8* }* noalias) unnamed_addr #0

; Function Attrs: nounwind readnone
define internal void @_ZN4main14as_closure.983E(i8* nocapture) unnamed_addr #1 {
entry-block:
  ret void
}

; Function Attrs: nounwind
declare void @llvm.lifetime.start(i64, i8* nocapture) #2

; Function Attrs: nounwind
declare void @llvm.lifetime.end(i64, i8* nocapture) #2

attributes #0 = { "split-stack" }
attributes #1 = { nounwind readnone }
attributes #2 = { nounwind }
