
; Function Attrs: readonly uwtable
define i32 @dot_ref_s({ i32*, i64 }* noalias nocapture readonly dereferenceable(16), { i32*, i64 }* noalias nocapture readonly dereferenceable(16)) unnamed_addr #2 personality i32 (i32, i32, i64, %"8.unwind::libunwind::_Unwind_Exception"*, %"8.unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
entry-block:
  %2 = getelementptr inbounds { i32*, i64 }, { i32*, i64 }* %0, i64 0, i32 0
  %3 = getelementptr inbounds { i32*, i64 }, { i32*, i64 }* %0, i64 0, i32 1
  %4 = load i32*, i32** %2, align 8
  %5 = load i64, i64* %3, align 8
  %6 = getelementptr inbounds { i32*, i64 }, { i32*, i64 }* %1, i64 0, i32 0
  %7 = getelementptr inbounds { i32*, i64 }, { i32*, i64 }* %1, i64 0, i32 1
  %8 = load i32*, i32** %6, align 8
  %9 = load i64, i64* %7, align 8
  %10 = icmp ule i64 %5, %9
  %_0.0.sroa.speculated.i.i.i = select i1 %10, i64 %5, i64 %9
  %11 = icmp eq i64 %_0.0.sroa.speculated.i.i.i, 0
  br i1 %11, label %bb6, label %"_ZN84_$LT$core..iter..Zip$LT$A$C$$u20$B$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4next17h91c35d8ab9003ffeE.exit.preheader"

"_ZN84_$LT$core..iter..Zip$LT$A$C$$u20$B$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4next17h91c35d8ab9003ffeE.exit.preheader": ; preds = %entry-block
  br label %"_ZN84_$LT$core..iter..Zip$LT$A$C$$u20$B$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4next17h91c35d8ab9003ffeE.exit"

"_ZN84_$LT$core..iter..Zip$LT$A$C$$u20$B$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4next17h91c35d8ab9003ffeE.exit": ; preds = %"_ZN84_$LT$core..iter..Zip$LT$A$C$$u20$B$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4next17h91c35d8ab9003ffeE.exit.preheader", %bb7
  %s.017 = phi i32 [ %18, %bb7 ], [ 0, %"_ZN84_$LT$core..iter..Zip$LT$A$C$$u20$B$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4next17h91c35d8ab9003ffeE.exit.preheader" ]
  %iter.sroa.37.016 = phi i64 [ %14, %bb7 ], [ 0, %"_ZN84_$LT$core..iter..Zip$LT$A$C$$u20$B$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4next17h91c35d8ab9003ffeE.exit.preheader" ]
  %12 = getelementptr inbounds i32, i32* %4, i64 %iter.sroa.37.016
  %switchtmp = icmp eq i32* %12, null ; <- ********************************** NULL CHECK HERE
  br i1 %switchtmp, label %bb6.loopexit, label %bb7

bb6.loopexit:                                     ; preds = %bb7, %"_ZN84_$LT$core..iter..Zip$LT$A$C$$u20$B$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4next17h91c35d8ab9003ffeE.exit"
  %s.0.lcssa.ph = phi i32 [ %s.017, %"_ZN84_$LT$core..iter..Zip$LT$A$C$$u20$B$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4next17h91c35d8ab9003ffeE.exit" ], [ %18, %bb7 ]
  br label %bb6

bb6:                                              ; preds = %bb6.loopexit, %entry-block
  %s.0.lcssa = phi i32 [ 0, %entry-block ], [ %s.0.lcssa.ph, %bb6.loopexit ]
  ret i32 %s.0.lcssa

bb7:                                              ; preds = %"_ZN84_$LT$core..iter..Zip$LT$A$C$$u20$B$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4next17h91c35d8ab9003ffeE.exit"
  %13 = getelementptr inbounds i32, i32* %8, i64 %iter.sroa.37.016
  %14 = add nuw i64 %iter.sroa.37.016, 1
  %15 = load i32, i32* %12, align 4
  %16 = load i32, i32* %13, align 4
  %17 = mul i32 %16, %15
  %18 = add i32 %17, %s.017
  %19 = icmp ult i64 %14, %_0.0.sroa.speculated.i.i.i
  br i1 %19, label %"_ZN84_$LT$core..iter..Zip$LT$A$C$$u20$B$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4next17h91c35d8ab9003ffeE.exit", label %bb6.loopexit
}
