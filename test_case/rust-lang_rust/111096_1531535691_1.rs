llvm-ir
; ModuleID = 'test_c.46e36185cb05fdac-cgu.0'
source_filename = "test_c.46e36185cb05fdac-cgu.0"
target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-msvc"

; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable
define noundef i32 @cast(i64 noundef %v) unnamed_addr #0 {
start:
  %0 = trunc i64 %v to i32
  ret i32 %0
}

attributes #0 = { mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable "target-cpu"="x86-64" }

!llvm.module.flags = !{!0}

!0 = !{i32 8, !"PIC Level", i32 2}
