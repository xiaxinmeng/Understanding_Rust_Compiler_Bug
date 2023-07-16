
; Function Attrs: nounwind nonlazybind uwtable
define i64 @check_foo2() unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %tmp.i.i.i.i.i.i = alloca i64, align 8
  %src.i.i.i = alloca i64, align 8
  %tmp.i.i.i.i.i.i.i = alloca i64, align 8
  %src.i.i.i.i = alloca i64, align 8
  %iter1.i = alloca %"core::iter::adapters::Rev<core::ops::range::RangeInclusive<u64>>", align 8
  %iter.i = alloca { i64, i64 }, align 8
  %0 = bitcast { i64, i64 }* %iter.i to i8*
  call void @llvm.lifetime.start.p0i8(i64 16, i8* nonnull %0) #3
  %1 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %iter.i, i64 0, i32 0
  store i64 0, i64* %1, align 8
  %2 = getelementptr inbounds { i64, i64 }, { i64, i64 }* %iter.i, i64 0, i32 1
  store i64 100000, i64* %2, align 8
  %3 = bitcast i64* %src.i.i.i to i8*
  %4 = bitcast i64* %tmp.i.i.i.i.i.i to i8*
  %_7.i.i.i.i.i.i.i.i = ptrtoint { i64, i64 }* %iter.i to i64
  %_7.i1.i.i.i.i.i.i.i = ptrtoint i64* %tmp.i.i.i.i.i.i to i64
  %_13.i.i.i.i.i.i.i1.i = icmp ult i64* %tmp.i.i.i.i.i.i, %1
  %5 = sub i64 %_7.i.i.i.i.i.i.i.i, %_7.i1.i.i.i.i.i.i.i
  %6 = sub i64 %_7.i1.i.i.i.i.i.i.i, %_7.i.i.i.i.i.i.i.i
  %diff.0.i.i.i.i.i.i.i2.i = select i1 %_13.i.i.i.i.i.i.i1.i, i64 %5, i64 %6
  %7 = icmp eq i64 %diff.0.i.i.i.i.i.i.i2.i, 0
  %_7.i.i4.i.i.i.i.i = ptrtoint i64* %src.i.i.i to i64
  %_13.i.i.i.i.i.i.i = icmp ugt i64* %src.i.i.i, %1
  %8 = sub i64 %_7.i.i4.i.i.i.i.i, %_7.i.i.i.i.i.i.i.i
  %9 = sub i64 %_7.i.i.i.i.i.i.i.i, %_7.i.i4.i.i.i.i.i
  %diff.0.i.i.i.i.i.i.i = select i1 %_13.i.i.i.i.i.i.i, i64 %8, i64 %9
  %10 = icmp eq i64 %diff.0.i.i.i.i.i.i.i, 0
  %11 = bitcast %"core::iter::adapters::Rev<core::ops::range::RangeInclusive<u64>>"* %iter1.i to i8*
  %_13.sroa.0.sroa.0.0._13.sroa.0.0..sroa_cast.sroa_idx.i = getelementptr inbounds %"core::iter::adapters::Rev<core::ops::range::RangeInclusive<u64>>", %"core::iter::adapters::Rev<core::ops::range::RangeInclusive<u64>>"* %iter1.i, i64 0, i32 0, i64 0
  %_13.sroa.0.sroa.4.0._13.sroa.0.0..sroa_cast.sroa_idx32.i = getelementptr inbounds %"core::iter::adapters::Rev<core::ops::range::RangeInclusive<u64>>", %"core::iter::adapters::Rev<core::ops::range::RangeInclusive<u64>>"* %iter1.i, i64 0, i32 1, i32 3
  %_13.sroa.0.sroa.5.0._13.sroa.0.0..sroa_cast.sroa_idx.i = getelementptr inbounds %"core::iter::adapters::Rev<core::ops::range::RangeInclusive<u64>>", %"core::iter::adapters::Rev<core::ops::range::RangeInclusive<u64>>"* %iter1.i, i64 0, i32 1, i32 5
  %12 = bitcast i64* %src.i.i.i.i to i8*
  %13 = bitcast i64* %tmp.i.i.i.i.i.i.i to i8*
  %_7.i.i.i.i.i.i.i.i.i = ptrtoint i64* %_13.sroa.0.sroa.4.0._13.sroa.0.0..sroa_cast.sroa_idx32.i to i64
  %_7.i1.i.i.i.i.i.i.i.i = ptrtoint i64* %tmp.i.i.i.i.i.i.i to i64
  %_13.i.i.i.i.i.i.i.i.i = icmp ult i64* %tmp.i.i.i.i.i.i.i, %_13.sroa.0.sroa.4.0._13.sroa.0.0..sroa_cast.sroa_idx32.i
  %14 = sub i64 %_7.i.i.i.i.i.i.i.i.i, %_7.i1.i.i.i.i.i.i.i.i
  %15 = sub i64 %_7.i1.i.i.i.i.i.i.i.i, %_7.i.i.i.i.i.i.i.i.i
  %diff.0.i.i.i.i.i.i.i.i.i = select i1 %_13.i.i.i.i.i.i.i.i.i, i64 %14, i64 %15
  %16 = icmp eq i64 %diff.0.i.i.i.i.i.i.i.i.i, 0
  %_7.i.i4.i.i.i.i.i.i = ptrtoint i64* %src.i.i.i.i to i64
  %_13.i.i.i.i.i.i.i.i = icmp ugt i64* %src.i.i.i.i, %_13.sroa.0.sroa.4.0._13.sroa.0.0..sroa_cast.sroa_idx32.i
  %17 = sub i64 %_7.i.i4.i.i.i.i.i.i, %_7.i.i.i.i.i.i.i.i.i
  %18 = sub i64 %_7.i.i.i.i.i.i.i.i.i, %_7.i.i4.i.i.i.i.i.i
  %diff.0.i.i.i.i.i.i.i.i = select i1 %_13.i.i.i.i.i.i.i.i, i64 %17, i64 %18
  %19 = icmp eq i64 %diff.0.i.i.i.i.i.i.i.i, 0
  br label %bb3.i.i

bb3.i.i:                                          ; preds = %bb12.i, %start
  %count.048.i = phi i64 [ 0, %start ], [ %count.1.i, %bb12.i ]
  %_3.i.i47.i = phi i64 [ 0, %start ], [ %20, %bb12.i ]
  %20 = add nuw nsw i64 %_3.i.i47.i, 1
  call void @llvm.lifetime.start.p0i8(i64 8, i8* nonnull %3) #3
  store i64 %20, i64* %src.i.i.i, align 8
  call void @llvm.lifetime.start.p0i8(i64 8, i8* nonnull %4) #3
  br i1 %7, label %bb11.i.i.i.i.i.i3.i, label %_ZN4core3ptr4read17hfae0c561b8371784E.exit.i.i.i.i.i

bb11.i.i.i.i.i.i3.i:                              ; preds = %bb3.i.i
  call void @llvm.trap() #3
  unreachable

_ZN4core3ptr4read17hfae0c561b8371784E.exit.i.i.i.i.i: ; preds = %bb3.i.i
  call void @llvm.lifetime.end.p0i8(i64 8, i8* nonnull %4) #3
  br i1 %10, label %bb11.i.i.i.i.i.i, label %bb6.i

bb11.i.i.i.i.i.i:                                 ; preds = %_ZN4core3ptr4read17hfae0c561b8371784E.exit.i.i.i.i.i
  call void @llvm.trap() #3
  unreachable

bb6.i:                                            ; preds = %_ZN4core3ptr4read17hfae0c561b8371784E.exit.i.i.i.i.i
  store i64 %20, i64* %1, align 8
  call void @llvm.lifetime.end.p0i8(i64 8, i8* nonnull %3) #3
  call void @llvm.lifetime.start.p0i8(i64 24, i8* nonnull %11) #3
  store i64 0, i64* %_13.sroa.0.sroa.0.0._13.sroa.0.0..sroa_cast.sroa_idx.i, align 8
  store i64 100000, i64* %_13.sroa.0.sroa.4.0._13.sroa.0.0..sroa_cast.sroa_idx32.i, align 8
  store i8 0, i8* %_13.sroa.0.sroa.5.0._13.sroa.0.0..sroa_cast.sroa_idx.i, align 8
  br label %bb10.i

bb10.i:                                           ; preds = %bb14.i, %bb6.i
  %.pr56.i = phi i8 [ %.pr.i, %bb14.i ], [ 0, %bb6.i ]
  %_4.i.i.i.i.i = phi i64 [ %_4.i.i.i.i51.i, %bb14.i ], [ 100000, %bb6.i ]
  %count.1.i = phi i64 [ %25, %bb14.i ], [ %count.048.i, %bb6.i ]
  %_2.i.i.i.i = icmp eq i8 %.pr56.i, 0
  br i1 %_2.i.i.i.i, label %bb2.i.i.i, label %"_ZN93_$LT$core..iter..adapters..Rev$LT$I$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h13a5af312fc9e945E.exit.i"

bb2.i.i.i:                                        ; preds = %bb10.i
  %21 = icmp eq i64 %_4.i.i.i.i.i, 0
  br i1 %21, label %bb5.i.i.i, label %bb6.i.i.i

bb5.i.i.i:                                        ; preds = %bb2.i.i.i
  store i8 1, i8* %_13.sroa.0.sroa.5.0._13.sroa.0.0..sroa_cast.sroa_idx.i, align 8
  br label %bb11.i.i.i

bb6.i.i.i:                                        ; preds = %bb2.i.i.i
  %22 = add i64 %_4.i.i.i.i.i, -1
  call void @llvm.lifetime.start.p0i8(i64 8, i8* nonnull %12) #3
  store i64 %22, i64* %src.i.i.i.i, align 8
  call void @llvm.lifetime.start.p0i8(i64 8, i8* nonnull %13) #3
  br i1 %16, label %bb11.i.i.i.i.i.i.i.i, label %_ZN4core3ptr4read17hfae0c561b8371784E.exit.i.i.i.i.i.i

bb11.i.i.i.i.i.i.i.i:                             ; preds = %bb6.i.i.i
  call void @llvm.trap() #3
  unreachable

_ZN4core3ptr4read17hfae0c561b8371784E.exit.i.i.i.i.i.i: ; preds = %bb6.i.i.i
  call void @llvm.lifetime.end.p0i8(i64 8, i8* nonnull %13) #3
  br i1 %19, label %bb11.i.i.i.i.i.i.i, label %_ZN4core3mem7replace17he25457917febc21fE.exit.i.i.i

bb11.i.i.i.i.i.i.i:                               ; preds = %_ZN4core3ptr4read17hfae0c561b8371784E.exit.i.i.i.i.i.i
  call void @llvm.trap() #3
  unreachable

_ZN4core3mem7replace17he25457917febc21fE.exit.i.i.i: ; preds = %_ZN4core3ptr4read17hfae0c561b8371784E.exit.i.i.i.i.i.i
  store i64 %22, i64* %_13.sroa.0.sroa.4.0._13.sroa.0.0..sroa_cast.sroa_idx32.i, align 8
  call void @llvm.lifetime.end.p0i8(i64 8, i8* nonnull %12) #3
  br label %bb11.i.i.i

bb11.i.i.i:                                       ; preds = %_ZN4core3mem7replace17he25457917febc21fE.exit.i.i.i, %bb5.i.i.i
  %.pr55.i = phi i8 [ 0, %_ZN4core3mem7replace17he25457917febc21fE.exit.i.i.i ], [ 1, %bb5.i.i.i ]
  %_4.i.i.i.i52.i = phi i64 [ %22, %_ZN4core3mem7replace17he25457917febc21fE.exit.i.i.i ], [ 0, %bb5.i.i.i ]
  %23 = insertvalue { i64, i64 } { i64 1, i64 undef }, i64 %_4.i.i.i.i.i, 1
  br label %"_ZN93_$LT$core..iter..adapters..Rev$LT$I$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h13a5af312fc9e945E.exit.i"

"_ZN93_$LT$core..iter..adapters..Rev$LT$I$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h13a5af312fc9e945E.exit.i": ; preds = %bb11.i.i.i, %bb10.i
  %.pr.i = phi i8 [ %.pr55.i, %bb11.i.i.i ], [ %.pr56.i, %bb10.i ]
  %_4.i.i.i.i51.i = phi i64 [ %_4.i.i.i.i52.i, %bb11.i.i.i ], [ %_4.i.i.i.i.i, %bb10.i ]
  %24 = phi { i64, i64 } [ %23, %bb11.i.i.i ], [ { i64 0, i64 undef }, %bb10.i ]
  %.fca.0.extract13.i = extractvalue { i64, i64 } %24, 0
  %switch4.i = icmp eq i64 %.fca.0.extract13.i, 0
  br i1 %switch4.i, label %bb12.i, label %bb14.i

bb12.i:                                           ; preds = %"_ZN93_$LT$core..iter..adapters..Rev$LT$I$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h13a5af312fc9e945E.exit.i"
  call void @llvm.lifetime.end.p0i8(i64 24, i8* nonnull %11) #3
  %exitcond = icmp eq i64 %20, 100000
  br i1 %exitcond, label %_ZN11issue_452224foo217h0cf1bd0d89259221E.exit, label %bb3.i.i

bb14.i:                                           ; preds = %"_ZN93_$LT$core..iter..adapters..Rev$LT$I$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17h13a5af312fc9e945E.exit.i"
  %.fca.1.extract14.i = extractvalue { i64, i64 } %24, 1
  %25 = add i64 %.fca.1.extract14.i, %count.1.i
  br label %bb10.i

_ZN11issue_452224foo217h0cf1bd0d89259221E.exit:   ; preds = %bb12.i
  call void @llvm.lifetime.end.p0i8(i64 16, i8* nonnull %0) #3
  ret i64 %count.1.i
}

