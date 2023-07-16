
define internal i32 @"_ZN4core3num20_$LT$impl$u20$u8$GT$14trailing_zeros17hf3376a11a840d3c1E"(i8 %self) unnamed_addr #0 !dbg !5 {
  %tmp_ret = alloca i8, align 1
  %0 = call i8 @llvm.cttz.i8(i8 %self, i1 false), !dbg !11
  store i8 %0, i8* %tmp_ret, align 1, !dbg !11
  %1 = load i8, i8* %tmp_ret, align 1, !dbg !11
  br label %bb1, !dbg !11

bb1:                                              ; preds = %start
  %2 = zext i8 %1 to i32, !dbg !13
  ret i32 %2, !dbg !14
}
