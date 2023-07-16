 llvm

declare void @foo();

; Function Attrs: uwtable
define external void @main() unnamed_addr #6 {
entry-block:
  %value.i = alloca i32, align 4
  %0 = bitcast i32* %value.i to i8*
  store i32 5, i32* %value.i, align 4

  call void asm "", "r,~{dirflag},~{fpsr},~{flags}"(i32* nonnull %value.i) #7

  %1 = load i32, i32* %value.i, align 4
  switch i32 %1, label %join [
    i32 1, label %match_case
    i32 48, label %match_case11
    i32 92, label %match_case12
    i32 23, label %match_case13
    i32 4, label %match_case14
    i32 70, label %match_case15
    i32 29, label %match_case16
    i32 36, label %match_case17
    i32 34, label %match_case18
    i32 58, label %match_case19
  ]

match_case:                                       ; preds = %entry-block
  call void @foo()
  unreachable

match_case11:                                     ; preds = %entry-block
  call void @foo()
  unreachable

match_case12:                                     ; preds = %entry-block
  call void @foo()
  unreachable

match_case13:                                     ; preds = %entry-block
  call void @foo()
  unreachable

match_case14:                                     ; preds = %entry-block
  call void @foo()
  unreachable

match_case15:                                     ; preds = %entry-block
  call void @foo()
  unreachable

match_case16:                                     ; preds = %entry-block
  call void @foo()
  unreachable

match_case17:                                     ; preds = %entry-block
  call void @foo()
  unreachable

match_case18:                                     ; preds = %entry-block
  call void @foo()
  unreachable

match_case19:                                     ; preds = %entry-block
  call void @foo()
  unreachable

join:                                             ; preds = %entry-block
  ret void
}
