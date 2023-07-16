
define zeroext i1 @"_ZN49_$LT$core..slice..Iter$LT$$u27$a$C$$u20$T$GT$$GT$12search_while_bad17hc287fe675e01e0b8E"(%"core::slice::Iter<i32>"* nocapture dereferenceable(16), i1 zeroext) unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %_0.sroa.0.i164 = alloca i8, align 1
  %_0.sroa.3.i165 = alloca i8, align 1
  %_0.sroa.0.i153 = alloca i8, align 1
  %_0.sroa.3.i154 = alloca i8, align 1
  %_0.sroa.0.i142 = alloca i8, align 1
  %_0.sroa.3.i143 = alloca i8, align 1
  %_0.sroa.0.i131 = alloca i8, align 1
  %_0.sroa.3.i132 = alloca i8, align 1
  %_0.sroa.0.i = alloca i8, align 1
  %_0.sroa.3.i = alloca i8, align 1
  %2 = getelementptr inbounds %"core::slice::Iter<i32>", %"core::slice::Iter<i32>"* %0, i64 0, i32 0
  %3 = getelementptr inbounds %"core::slice::Iter<i32>", %"core::slice::Iter<i32>"* %0, i64 0, i32 2
  %4 = bitcast i32** %3 to i64*
  %.pre = load i32*, i32** %2, align 8
  %.pre180 = load i64, i64* %4, align 8
  %5 = inttoptr i64 %.pre180 to i32*
  br label %bb6

bb6:                                              ; preds = %start, %bb56
  %6 = phi i32* [ %.pre, %start ], [ %22, %bb56 ]
  %7 = ptrtoint i32* %6 to i64
  %8 = sub i64 %.pre180, %7
  %9 = sdiv i64 %8, 4
  %10 = icmp ugt i64 %9, 3
  br i1 %10, label %bb14, label %bb61.preheader

bb61.preheader:                                   ; preds = %bb6
  br label %bb61

bb14:                                             ; preds = %bb6
  %11 = getelementptr inbounds i32, i32* %6, i64 1
  store i32* %11, i32** %2, align 8
  %12 = icmp ne i32* %6, null
  tail call void @llvm.assume(i1 %12)
  call void @llvm.lifetime.start(i64 1, i8* nonnull %_0.sroa.0.i164)
  call void @llvm.lifetime.start(i64 1, i8* nonnull %_0.sroa.3.i165)
  %13 = load i32, i32* %6, align 4
  %14 = icmp eq i32 %13, 1
  br i1 %14, label %bb3.i166, label %bb15

bb3.i166:                                         ; preds = %bb14
  store i8 1, i8* %_0.sroa.0.i164, align 1
  br label %bb15

bb15:                                             ; preds = %bb3.i166, %bb14
  %.sink.i167 = phi i8* [ %_0.sroa.3.i165, %bb3.i166 ], [ %_0.sroa.0.i164, %bb14 ]
  store i8 0, i8* %.sink.i167, align 1
  %_0.sroa.0.i164.0._0.sroa.0.0._0.sroa.0.0._0.sroa.0.0..i168 = load i8, i8* %_0.sroa.0.i164, align 1
  %_0.sroa.3.i165.0._0.sroa.3.0._0.sroa.3.0._0.sroa.3.0..i169 = load i8, i8* %_0.sroa.3.i165, align 1
  %_0.sroa.3.0.insert.ext.i170 = zext i8 %_0.sroa.3.i165.0._0.sroa.3.0._0.sroa.3.0._0.sroa.3.0..i169 to i16
  %_0.sroa.3.0.insert.shift.i171 = shl nuw i16 %_0.sroa.3.0.insert.ext.i170, 8
  %_0.sroa.0.0.insert.ext.i172 = zext i8 %_0.sroa.0.i164.0._0.sroa.0.0._0.sroa.0.0._0.sroa.0.0..i168 to i16
  %_0.sroa.0.0.insert.insert.i173 = or i16 %_0.sroa.3.0.insert.shift.i171, %_0.sroa.0.0.insert.ext.i172
  call void @llvm.lifetime.end(i64 1, i8* nonnull %_0.sroa.0.i164)
  call void @llvm.lifetime.end(i64 1, i8* nonnull %_0.sroa.3.i165)
  %cond2 = icmp eq i8 %_0.sroa.0.i164.0._0.sroa.0.0._0.sroa.0.0._0.sroa.0.0..i168, 0
  br i1 %cond2, label %bb31, label %bb22.loopexit185

bb19:                                             ; preds = %bb62, %bb22
  %_0.0 = phi i8 [ %_0.1, %bb22 ], [ %27, %bb62 ]
  %15 = icmp ne i8 %_0.0, 0
  ret i1 %15

bb22.loopexit:                                    ; preds = %bb71
  %_0.sroa.3.0.insert.ext.i.le = zext i8 %_0.sroa.3.i.0._0.sroa.3.0._0.sroa.3.0._0.sroa.3.0..i to i16
  %_0.sroa.3.0.insert.shift.i.le = shl nuw i16 %_0.sroa.3.0.insert.ext.i.le, 8
  %_0.sroa.0.0.insert.ext.i.le = zext i8 %_0.sroa.0.i.0._0.sroa.0.0._0.sroa.0.0._0.sroa.0.0..i to i16
  %_0.sroa.0.0.insert.insert.i.le = or i16 %_0.sroa.3.0.insert.shift.i.le, %_0.sroa.0.0.insert.ext.i.le
  br label %bb22

bb22.loopexit185:                                 ; preds = %bb15, %bb32, %bb44, %bb56
  %_0.1.in.in.in.ph = phi i16 [ %_0.sroa.0.0.insert.insert.i140, %bb56 ], [ %_0.sroa.0.0.insert.insert.i151, %bb44 ], [ %_0.sroa.0.0.insert.insert.i162, %bb32 ], [ %_0.sroa.0.0.insert.insert.i173, %bb15 ]
  br label %bb22

bb22:                                             ; preds = %bb22.loopexit185, %bb22.loopexit
  %_0.1.in.in.in = phi i16 [ %_0.sroa.0.0.insert.insert.i.le, %bb22.loopexit ], [ %_0.1.in.in.in.ph, %bb22.loopexit185 ]
  %_0.1.in.in = lshr i16 %_0.1.in.in.in, 8
  %_0.1.in = trunc i16 %_0.1.in.in to i8
  %_0.1 = and i8 %_0.1.in, 1
  br label %bb19

bb31:                                             ; preds = %bb15
  %16 = getelementptr inbounds i32, i32* %6, i64 2
  store i32* %16, i32** %2, align 8
  call void @llvm.lifetime.start(i64 1, i8* nonnull %_0.sroa.0.i153)
  call void @llvm.lifetime.start(i64 1, i8* nonnull %_0.sroa.3.i154)
  %17 = load i32, i32* %11, align 4
  %18 = icmp eq i32 %17, 1
  br i1 %18, label %bb3.i155, label %bb32

bb3.i155:                                         ; preds = %bb31
  store i8 1, i8* %_0.sroa.0.i153, align 1
  br label %bb32

bb32:                                             ; preds = %bb3.i155, %bb31
  %.sink.i156 = phi i8* [ %_0.sroa.3.i154, %bb3.i155 ], [ %_0.sroa.0.i153, %bb31 ]
  store i8 0, i8* %.sink.i156, align 1
  %_0.sroa.0.i153.0._0.sroa.0.0._0.sroa.0.0._0.sroa.0.0..i157 = load i8, i8* %_0.sroa.0.i153, align 1
  %_0.sroa.3.i154.0._0.sroa.3.0._0.sroa.3.0._0.sroa.3.0..i158 = load i8, i8* %_0.sroa.3.i154, align 1
  %_0.sroa.3.0.insert.ext.i159 = zext i8 %_0.sroa.3.i154.0._0.sroa.3.0._0.sroa.3.0._0.sroa.3.0..i158 to i16
  %_0.sroa.3.0.insert.shift.i160 = shl nuw i16 %_0.sroa.3.0.insert.ext.i159, 8
  %_0.sroa.0.0.insert.ext.i161 = zext i8 %_0.sroa.0.i153.0._0.sroa.0.0._0.sroa.0.0._0.sroa.0.0..i157 to i16
  %_0.sroa.0.0.insert.insert.i162 = or i16 %_0.sroa.3.0.insert.shift.i160, %_0.sroa.0.0.insert.ext.i161
  call void @llvm.lifetime.end(i64 1, i8* nonnull %_0.sroa.0.i153)
  call void @llvm.lifetime.end(i64 1, i8* nonnull %_0.sroa.3.i154)
  %cond4 = icmp eq i8 %_0.sroa.0.i153.0._0.sroa.0.0._0.sroa.0.0._0.sroa.0.0..i157, 0
  br i1 %cond4, label %bb43, label %bb22.loopexit185

bb43:                                             ; preds = %bb32
  %19 = getelementptr inbounds i32, i32* %6, i64 3
  store i32* %19, i32** %2, align 8
  call void @llvm.lifetime.start(i64 1, i8* nonnull %_0.sroa.0.i142)
  call void @llvm.lifetime.start(i64 1, i8* nonnull %_0.sroa.3.i143)
  %20 = load i32, i32* %16, align 4
  %21 = icmp eq i32 %20, 1
  br i1 %21, label %bb3.i144, label %bb44

bb3.i144:                                         ; preds = %bb43
  store i8 1, i8* %_0.sroa.0.i142, align 1
  br label %bb44

bb44:                                             ; preds = %bb3.i144, %bb43
  %.sink.i145 = phi i8* [ %_0.sroa.3.i143, %bb3.i144 ], [ %_0.sroa.0.i142, %bb43 ]
  store i8 0, i8* %.sink.i145, align 1
  %_0.sroa.0.i142.0._0.sroa.0.0._0.sroa.0.0._0.sroa.0.0..i146 = load i8, i8* %_0.sroa.0.i142, align 1
  %_0.sroa.3.i143.0._0.sroa.3.0._0.sroa.3.0._0.sroa.3.0..i147 = load i8, i8* %_0.sroa.3.i143, align 1
  %_0.sroa.3.0.insert.ext.i148 = zext i8 %_0.sroa.3.i143.0._0.sroa.3.0._0.sroa.3.0._0.sroa.3.0..i147 to i16
  %_0.sroa.3.0.insert.shift.i149 = shl nuw i16 %_0.sroa.3.0.insert.ext.i148, 8
  %_0.sroa.0.0.insert.ext.i150 = zext i8 %_0.sroa.0.i142.0._0.sroa.0.0._0.sroa.0.0._0.sroa.0.0..i146 to i16
  %_0.sroa.0.0.insert.insert.i151 = or i16 %_0.sroa.3.0.insert.shift.i149, %_0.sroa.0.0.insert.ext.i150
  call void @llvm.lifetime.end(i64 1, i8* nonnull %_0.sroa.0.i142)
  call void @llvm.lifetime.end(i64 1, i8* nonnull %_0.sroa.3.i143)
  %cond6 = icmp eq i8 %_0.sroa.0.i142.0._0.sroa.0.0._0.sroa.0.0._0.sroa.0.0..i146, 0
  br i1 %cond6, label %bb55, label %bb22.loopexit185

bb55:                                             ; preds = %bb44
  %22 = getelementptr inbounds i32, i32* %6, i64 4
  store i32* %22, i32** %2, align 8
  call void @llvm.lifetime.start(i64 1, i8* nonnull %_0.sroa.0.i131)
  call void @llvm.lifetime.start(i64 1, i8* nonnull %_0.sroa.3.i132)
  %23 = load i32, i32* %19, align 4
  %24 = icmp eq i32 %23, 1
  br i1 %24, label %bb3.i133, label %bb56

bb3.i133:                                         ; preds = %bb55
  store i8 1, i8* %_0.sroa.0.i131, align 1
  br label %bb56

bb56:                                             ; preds = %bb3.i133, %bb55
  %.sink.i134 = phi i8* [ %_0.sroa.3.i132, %bb3.i133 ], [ %_0.sroa.0.i131, %bb55 ]
  store i8 0, i8* %.sink.i134, align 1
  %_0.sroa.0.i131.0._0.sroa.0.0._0.sroa.0.0._0.sroa.0.0..i135 = load i8, i8* %_0.sroa.0.i131, align 1
  %_0.sroa.3.i132.0._0.sroa.3.0._0.sroa.3.0._0.sroa.3.0..i136 = load i8, i8* %_0.sroa.3.i132, align 1
  %_0.sroa.3.0.insert.ext.i137 = zext i8 %_0.sroa.3.i132.0._0.sroa.3.0._0.sroa.3.0._0.sroa.3.0..i136 to i16
  %_0.sroa.3.0.insert.shift.i138 = shl nuw i16 %_0.sroa.3.0.insert.ext.i137, 8
  %_0.sroa.0.0.insert.ext.i139 = zext i8 %_0.sroa.0.i131.0._0.sroa.0.0._0.sroa.0.0._0.sroa.0.0..i135 to i16
  %_0.sroa.0.0.insert.insert.i140 = or i16 %_0.sroa.3.0.insert.shift.i138, %_0.sroa.0.0.insert.ext.i139
  call void @llvm.lifetime.end(i64 1, i8* nonnull %_0.sroa.0.i131)
  call void @llvm.lifetime.end(i64 1, i8* nonnull %_0.sroa.3.i132)
  %cond8 = icmp eq i8 %_0.sroa.0.i131.0._0.sroa.0.0._0.sroa.0.0._0.sroa.0.0..i135, 0
  br i1 %cond8, label %bb6, label %bb22.loopexit185

bb61:                                             ; preds = %bb61.preheader, %bb71
  %25 = phi i32* [ %28, %bb71 ], [ %6, %bb61.preheader ]
  %26 = icmp eq i32* %25, %5
  br i1 %26, label %bb62, label %bb70

bb62:                                             ; preds = %bb61
  %27 = zext i1 %1 to i8
  br label %bb19

bb70:                                             ; preds = %bb61
  %28 = getelementptr inbounds i32, i32* %25, i64 1
  store i32* %28, i32** %2, align 8
  %29 = icmp ne i32* %25, null
  tail call void @llvm.assume(i1 %29)
  call void @llvm.lifetime.start(i64 1, i8* nonnull %_0.sroa.0.i)
  call void @llvm.lifetime.start(i64 1, i8* nonnull %_0.sroa.3.i)
  %30 = load i32, i32* %25, align 4
  %31 = icmp eq i32 %30, 1
  br i1 %31, label %bb3.i, label %bb71

bb3.i:                                            ; preds = %bb70
  store i8 1, i8* %_0.sroa.0.i, align 1
  br label %bb71

bb71:                                             ; preds = %bb3.i, %bb70
  %.sink.i = phi i8* [ %_0.sroa.3.i, %bb3.i ], [ %_0.sroa.0.i, %bb70 ]
  store i8 0, i8* %.sink.i, align 1
  %_0.sroa.0.i.0._0.sroa.0.0._0.sroa.0.0._0.sroa.0.0..i = load i8, i8* %_0.sroa.0.i, align 1
  %_0.sroa.3.i.0._0.sroa.3.0._0.sroa.3.0._0.sroa.3.0..i = load i8, i8* %_0.sroa.3.i, align 1
  call void @llvm.lifetime.end(i64 1, i8* nonnull %_0.sroa.0.i)
  call void @llvm.lifetime.end(i64 1, i8* nonnull %_0.sroa.3.i)
  %cond = icmp eq i8 %_0.sroa.0.i.0._0.sroa.0.0._0.sroa.0.0._0.sroa.0.0..i, 0
  br i1 %cond, label %bb61, label %bb22.loopexit
}
