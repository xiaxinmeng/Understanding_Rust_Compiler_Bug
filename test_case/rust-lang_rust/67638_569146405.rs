
; core::ptr::<impl *const T>::offset_from
; Function Attrs: inlinehint uwtable
define internal i64 @"_ZN4core3ptr33_$LT$impl$u20$$BP$const$u20$T$GT$11offset_from17h5d75b21cc3f5b3e2E"(%NonOrd* %self, %NonOrd* %origin) unnamed_addr #0 {
start:
  %0 = alloca i64, align 8
  %1 = alloca i64, align 8
  %ok = alloca i8, align 1
  store i64 0, i64* %0, align 8
  %2 = load i64, i64* %0, align 8
  br label %bb1

bb1:                                              ; preds = %start
  %_5 = icmp ult i64 0, %2
  br i1 %_5, label %bb4, label %bb3

bb2:                                              ; preds = %bb6
  store i8 1, i8* %ok, align 1
  br label %bb5

bb3:                                              ; preds = %bb6, %bb1
  store i8 0, i8* %ok, align 1
  br label %bb5

bb4:                                              ; preds = %bb1
  br label %bb6

bb5:                                              ; preds = %bb3, %bb2
  %3 = load i8, i8* %ok, align 1, !range !12
  %_13 = trunc i8 %3 to i1
  %_12 = xor i1 %_13, true
  %4 = icmp ule i1 %_12, true
  call void @llvm.assume(i1 %4)
  %_11 = zext i1 %_12 to i64
  %_15 = icmp ult i64 %_11, 1
  %5 = call i1 @llvm.expect.i1(i1 %_15, i1 true)
  br i1 %5, label %bb7, label %panic

bb6:                                              ; preds = %bb4
  %_7 = icmp ule i64 %2, 9223372036854775807
  br i1 %_7, label %bb2, label %bb3

bb7:                                              ; preds = %bb5
  %6 = ptrtoint %NonOrd* %self to i64
  %7 = ptrtoint %NonOrd* %origin to i64
  %8 = sub i64 %6, %7
  %9 = sdiv exact i64 %8, 0
  store i64 %9, i64* %1, align 8
  %10 = load i64, i64* %1, align 8
  br label %bb8
