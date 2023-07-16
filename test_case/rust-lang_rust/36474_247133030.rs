 llvm
; Function Attrs: nounwind readnone uwtable
define internal fastcc void @_ZN4test11remove_axis17h518617d2aa05c61eE() unnamed_addr #1 personality i32 (i32, i32, i64, %"8.unwind::libunwind::_Unwind_Exception"*, %"8.unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
entry-block:
  br label %bb7

bb7:                                              ; preds = %bb10.i, %bb12, %bb10, %entry-block
  %iter.sroa.0.0 = phi i64* [ @ref8151, %entry-block ], [ %iter.sroa.0.1, %bb10 ], [ %iter.sroa.0.1, %bb12 ], [ %iter.sroa.0.1, %bb10.i ]
  %iter.sroa.7.0 = phi i64 [ 0, %entry-block ], [ %iter.sroa.7.1, %bb10 ], [ %iter.sroa.7.1, %bb12 ], [ %iter.sroa.7.1, %bb10.i ]
  %it.sroa.0.0 = phi i64* [ getelementptr inbounds ([0 x i64], [0 x i64]* @ref8152, i64 0, i64 0), %entry-block ], [ %it.sroa.0.0, %bb10 ], [ getelementptr inbounds ([0 x i64], [0 x i64]* @ref8152, i64 0, i64 0), %bb12 ], [ %7, %bb10.i ]
  %tmp10.sroa.0.0 = phi i64 [ undef, %entry-block ], [ 0, %bb10 ], [ %tmp10.sroa.0.1, %bb12 ], [ %tmp10.sroa.0.1, %bb10.i ]
  %0 = icmp eq i64* %iter.sroa.0.0, getelementptr inbounds (i64, i64* @ref8151, i64 1)
  br i1 %0, label %"_ZN81_$LT$core..iter..Enumerate$LT$I$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4next17h32f5bf83064acb8cE.exit", label %"_ZN91_$LT$core..slice..Iter$LT$$u27$a$C$$u20$T$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4next17hbcf540387e7bb34eE.exit.i"

"_ZN91_$LT$core..slice..Iter$LT$$u27$a$C$$u20$T$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4next17hbcf540387e7bb34eE.exit.i": ; preds = %bb7
  %1 = getelementptr inbounds i64, i64* %iter.sroa.0.0, i64 1
  %2 = add i64 %iter.sroa.7.0, 1
  %3 = ptrtoint i64* %iter.sroa.0.0 to i64
  br label %"_ZN81_$LT$core..iter..Enumerate$LT$I$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4next17h32f5bf83064acb8cE.exit"

"_ZN81_$LT$core..iter..Enumerate$LT$I$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4next17h32f5bf83064acb8cE.exit": ; preds = %bb7, %"_ZN91_$LT$core..slice..Iter$LT$$u27$a$C$$u20$T$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4next17hbcf540387e7bb34eE.exit.i"
  %iter.sroa.0.1 = phi i64* [ %1, %"_ZN91_$LT$core..slice..Iter$LT$$u27$a$C$$u20$T$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4next17hbcf540387e7bb34eE.exit.i" ], [ getelementptr inbounds (i64, i64* @ref8151, i64 1), %bb7 ]
  %iter.sroa.7.1 = phi i64 [ %2, %"_ZN91_$LT$core..slice..Iter$LT$$u27$a$C$$u20$T$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4next17hbcf540387e7bb34eE.exit.i" ], [ %iter.sroa.7.0, %bb7 ]
  %tmp10.sroa.6.0 = phi i64 [ %3, %"_ZN91_$LT$core..slice..Iter$LT$$u27$a$C$$u20$T$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4next17hbcf540387e7bb34eE.exit.i" ], [ 0, %bb7 ]
  %tmp10.sroa.0.1 = phi i64 [ %iter.sroa.7.0, %"_ZN91_$LT$core..slice..Iter$LT$$u27$a$C$$u20$T$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4next17hbcf540387e7bb34eE.exit.i" ], [ %tmp10.sroa.0.0, %bb7 ]
  %4 = inttoptr i64 %tmp10.sroa.6.0 to i64*
  %switchtmp = icmp eq i64* %4, null
  br i1 %switchtmp, label %bb9, label %bb10

bb9:                                              ; preds = %"_ZN81_$LT$core..iter..Enumerate$LT$I$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4next17h32f5bf83064acb8cE.exit"
  ret void

bb10:                                             ; preds = %"_ZN81_$LT$core..iter..Enumerate$LT$I$GT$$u20$as$u20$core..iter..iterator..Iterator$GT$4next17h32f5bf83064acb8cE.exit"
  %5 = icmp eq i64 %tmp10.sroa.0.1, 0
  br i1 %5, label %bb7, label %bb12

bb12:                                             ; preds = %bb10
  %6 = icmp eq i64* %it.sroa.0.0, getelementptr inbounds ([0 x i64], [0 x i64]* @ref8152, i64 0, i64 0)
  br i1 %6, label %bb7, label %bb10.i

bb10.i:                                           ; preds = %bb12
  %7 = getelementptr inbounds i64, i64* %it.sroa.0.0, i64 1
  br label %bb7
}
