
; ModuleID = 'test.rs'
target datalayout = "e-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

; Function Attrs: uwtable
define internal i64 @_ZN5start20ha41e9c28898510a9faa4v0.0E(i64, i8**) unnamed_addr #0 {
entry-block:
  %argc = alloca i64
  %argv = alloca i8**
  %__adjust = alloca { void (i8*)*, i8* }
  store i64 %0, i64* %argc
  store i8** %1, i8*** %argv
  %2 = load i64* %argc
  %3 = load i8*** %argv
  %4 = getelementptr inbounds { void (i8*)*, i8* }* %__adjust, i32 0, i32 0
  store void (i8*)* @_ZN4main14as_closure.983E, void (i8*)** %4
  %5 = getelementptr inbounds { void (i8*)*, i8* }* %__adjust, i32 0, i32 1
  store i8* null, i8** %5
  %6 = call i64 @_ZN5start20h52ed3dec7308a2ebxzd9v0.11.preE(i64 %2, i8** %3, { void (i8*)*, i8* }* nocapture %__adjust)
  ret i64 %6
}

define i64 @main(i64, i8**) unnamed_addr {
top:
  %2 = call i64 @_ZN5start20ha41e9c28898510a9faa4v0.0E(i64 %0, i8** %1)
  ret i64 %2
}

declare i64 @_ZN5start20h52ed3dec7308a2ebxzd9v0.11.preE(i64, i8**, { void (i8*)*, i8* }* noalias) unnamed_addr #1

; Function Attrs: uwtable
define internal void @_ZN4main20ha1e063e2a65e22b5yaa4v0.0E() unnamed_addr #0 {
entry-block:
  ret void
}

define internal void @_ZN4main14as_closure.983E(i8*) unnamed_addr {
entry-block:
  call void @_ZN4main20ha1e063e2a65e22b5yaa4v0.0E()
  ret void
}

attributes #0 = { uwtable "split-stack" }
attributes #1 = { "split-stack" }
