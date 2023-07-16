llvm
; ModuleID = 'lib.f2c86790-cgu.0'
source_filename = "lib.f2c86790-cgu.0"
target datalayout = "e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64"
target triple = "thumbv6m-none-unknown-eabi"

; lib::Decimal::checked_mul
; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn
define dso_local { i32, i32 } @_ZN3lib7Decimal11checked_mul17hbf678d4f3ee90eb3E(i32 %self.0, i32 %self.1) unnamed_addr #0 {
start:
  %_3 = zext i32 %self.1 to i64
  %0 = mul nuw i64 %_3, %_3
  %_13 = lshr i64 %0, 32
  %1 = add nuw i64 %_13, %0
  %2 = add i64 %1, %0
  %_18 = icmp ult i64 %2, %1
  %_21 = lshr i64 %2, 32
  %3 = or i64 %_21, 4294967296
  %tmp2.0 = select i1 %_18, i64 %3, i64 %_21
  %4 = add i64 %tmp2.0, %0
  %_31.not = icmp eq i32 %self.1, 0
  %extract.t13 = trunc i64 %4 to i32
  %extract.t15 = trunc i64 %tmp2.0 to i32
  br i1 %_31.not, label %bb8, label %bb6

bb8:                                              ; preds = %bb6, %start
  %tmp.0.off0 = phi i32 [ %extract.t, %bb6 ], [ %extract.t13, %start ]
  %tmp2.1.off0 = phi i32 [ %extract.t14, %bb6 ], [ %extract.t15, %start ]
  %5 = insertvalue { i32, i32 } undef, i32 %tmp2.1.off0, 0
  %6 = insertvalue { i32, i32 } %5, i32 %tmp.0.off0, 1
  ret { i32, i32 } %6

bb6:                                              ; preds = %start
  %_33 = zext i32 %self.0 to i64
  %7 = mul nuw i64 %_3, %_33
  %8 = add i64 %4, %7
  %extract.t = trunc i64 %8 to i32
  %extract.t14 = trunc i64 %7 to i32
  br label %bb8
}

attributes #0 = { mustprogress nofree norecurse nosync nounwind readnone willreturn "frame-pointer"="all" "target-cpu"="generic" }
