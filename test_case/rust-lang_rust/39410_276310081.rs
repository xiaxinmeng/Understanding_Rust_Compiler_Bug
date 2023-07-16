llvm
; ModuleID = 'armv8_none_rbit.cgu-0.rs'
source_filename = "armv8_none_rbit.cgu-0.rs"
target datalayout = "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128"
target triple = "aarch64-unknown-linux-gnu"

; Function Attrs: nounwind readnone
define i64 @rbit_u64(i64) unnamed_addr #0 {
entry-block:
  %1 = tail call i64 @llvm.bitreverse.i64(i64 %0) #1
  ret i64 %1
}

; Function Attrs: nounwind readnone
declare i64 @llvm.bitreverse.i64(i64) unnamed_addr #0

attributes #0 = { nounwind readnone }
attributes #1 = { nounwind }
