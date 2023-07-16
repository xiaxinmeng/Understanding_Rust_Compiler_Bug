ll
; ModuleID = 'foo.cgu-0.rs'
source_filename = "foo.cgu-0.rs"
target datalayout = "e-p:32:32-i64:64-v128:32:128-n32-S128"
target triple = "asmjs-unknown-emscripten"

; Function Attrs: inlinehint uwtable
define internal i32 @"_ZN4core3num20_$LT$impl$u20$u8$GT$13leading_zeros17h0d6a1b849828bff0E"(i8) unnamed_addr #0 {
entry-block:
  %tmp_ret = alloca i8
  br label %start

start:                                            ; preds = %entry-block
  %1 = call i8 @llvm.ctlz.i8(i8 %0, i1 false)
  store i8 %1, i8* %tmp_ret
  %2 = load i8, i8* %tmp_ret
  br label %bb1

bb1:                                              ; preds = %start
  %3 = zext i8 %2 to i32
  ret i32 %3
}

; Function Attrs: uwtable
define i32 @_ZN3foo4main17ha0ecef40c3578f1eE() unnamed_addr #1 {
entry-block:
  br label %start

start:                                            ; preds = %entry-block
  %0 = call i32 @"_ZN4core3num20_$LT$impl$u20$u8$GT$13leading_zeros17h0d6a1b849828bff0E"(i8 -1)
  br label %bb1

bb1:                                              ; preds = %start
  ret i32 %0
}

; Function Attrs: nounwind readnone
declare i8 @llvm.ctlz.i8(i8, i1) #2

attributes #0 = { inlinehint uwtable }
attributes #1 = { uwtable }
attributes #2 = { nounwind readnone }
