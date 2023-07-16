

define zeroext i1 @"_ZN49_$LT$core..slice..Iter$LT$$u27$a$C$$u20$T$GT$$GT$12search_while_good17hc287fe675e01e0b8E"(%"core::slice::Iter<i32>"* nocapture dereferenceable(16), i1 zeroext) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %2 = getelementptr inbounds %"core::slice::Iter<i32>", %"core::slice::Iter<i32>"* %0, i64 0, i32 0
  %3 = getelementptr inbounds %"core::slice::Iter<i32>", %"core::slice::Iter<i32>"* %0, i64 0, i32 2
  %4 = bitcast i32** %3 to i64*
  %.pre = load i32*, i32** %2, align 8
  %.pre147 = load i64, i64* %4, align 8
  %5 = inttoptr i64 %.pre147 to i32*
  br label %bb6

bb6:                                              ; preds = %bb56, %start
  %6 = phi i32* [ %17, %bb56 ], [ %.pre, %start ]
  %7 = ptrtoint i32* %6 to i64
  %8 = sub i64 %.pre147, %7
  %9 = sdiv i64 %8, 4
  %10 = icmp ugt i64 %9, 3
  br i1 %10, label %bb15, label %bb61.preheader

bb61.preheader:                                   ; preds = %bb6
  br label %bb61

bb15:                                             ; preds = %bb6
  %11 = getelementptr inbounds i32, i32* %6, i64 1
  store i32* %11, i32** %2, align 8
  %12 = load i32, i32* %6, align 4
  %not..i140 = icmp eq i32 %12, 1
  br i1 %not..i140, label %bb19.loopexit152, label %bb32

bb19.loopexit:                                    ; preds = %bb71, %bb61
  %_0.0.ph = phi i1 [ %1, %bb61 ], [ false, %bb71 ]
  br label %bb19

bb19.loopexit152:                                 ; preds = %bb15, %bb32, %bb44, %bb56
  br label %bb19

bb19:                                             ; preds = %bb19.loopexit152, %bb19.loopexit
  %_0.0 = phi i1 [ %_0.0.ph, %bb19.loopexit ], [ false, %bb19.loopexit152 ]
  ret i1 %_0.0

bb32:                                             ; preds = %bb15
  %13 = getelementptr inbounds i32, i32* %6, i64 2
  store i32* %13, i32** %2, align 8
  %14 = load i32, i32* %11, align 4
  %not..i138 = icmp eq i32 %14, 1
  br i1 %not..i138, label %bb19.loopexit152, label %bb44

bb44:                                             ; preds = %bb32
  %15 = getelementptr inbounds i32, i32* %6, i64 3
  store i32* %15, i32** %2, align 8
  %16 = load i32, i32* %13, align 4
  %not..i136 = icmp eq i32 %16, 1
  br i1 %not..i136, label %bb19.loopexit152, label %bb56

bb56:                                             ; preds = %bb44
  %17 = getelementptr inbounds i32, i32* %6, i64 4
  store i32* %17, i32** %2, align 8
  %18 = load i32, i32* %15, align 4
  %not..i134 = icmp eq i32 %18, 1
  br i1 %not..i134, label %bb19.loopexit152, label %bb6

bb61:                                             ; preds = %bb61.preheader, %bb71
  %19 = phi i32* [ %21, %bb71 ], [ %6, %bb61.preheader ]
  %20 = icmp eq i32* %19, %5
  br i1 %20, label %bb19.loopexit, label %bb71

bb71:                                             ; preds = %bb61
  %21 = getelementptr inbounds i32, i32* %19, i64 1
  store i32* %21, i32** %2, align 8
  %22 = load i32, i32* %19, align 4
  %not..i = icmp eq i32 %22, 1
  br i1 %not..i, label %bb19.loopexit, label %bb61
}
