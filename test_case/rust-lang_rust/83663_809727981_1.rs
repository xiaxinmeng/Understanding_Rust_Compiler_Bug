llvm
; ModuleID = 'eq_test.3a1fbbbh-cgu.0'
source_filename = "eq_test.3a1fbbbh-cgu.0"
target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-msvc"

%Blueprint = type { [0 x i32], i32, [0 x i32], i32, [0 x i32], i32, [0 x i32], i32, [0 x i32], i32, [0 x i32] }

; <eq_test::Blueprint as core::cmp::PartialEq>::eq
; Function Attrs: norecurse nounwind readonly uwtable willreturn
define zeroext i1 @"_ZN59_$LT$eq_test..Blueprint$u20$as$u20$core..cmp..PartialEq$GT$2eq17he4e85086691c6c20E"(%Blueprint* noalias nocapture readonly align 4 dereferenceable(20) %self, %Blueprint* noalias nocapture readonly align 4 dereferenceable(20) %other) unnamed_addr #0 {
start:
  %0 = bitcast %Blueprint* %self to <4 x i32>*
  %1 = load <4 x i32>, <4 x i32>* %0, align 4
  %2 = bitcast %Blueprint* %other to <4 x i32>*
  %3 = load <4 x i32>, <4 x i32>* %2, align 4
  %4 = icmp eq <4 x i32> %1, %3
  %5 = getelementptr inbounds %Blueprint, %Blueprint* %self, i64 0, i32 9
  %_19 = load i32, i32* %5, align 4
  %6 = getelementptr inbounds %Blueprint, %Blueprint* %other, i64 0, i32 9
  %_20 = load i32, i32* %6, align 4
  %_18 = icmp eq i32 %_19, %_20
  %7 = call i1 @llvm.vector.reduce.and.v4i1(<4 x i1> %4)
  %8 = and i1 %7, %_18
  ret i1 %8
}

; Function Attrs: nofree nosync nounwind readnone willreturn
declare i1 @llvm.vector.reduce.and.v4i1(<4 x i1>) #1

attributes #0 = { norecurse nounwind readonly uwtable willreturn "target-cpu"="znver1" }
attributes #1 = { nofree nosync nounwind readnone willreturn }

!llvm.module.flags = !{!0}

!0 = !{i32 7, !"PIC Level", i32 2}
