llvm
; Function Attrs: uwtable
define void @print_if_some(i128 %0) unnamed_addr #3 {
  %2 = alloca i128, align 8
  %3 = bitcast i128* %2 to %"core::result::Result<u64, u32>"*
  store i128 %0, i128* %2, align 8
  %4 = getelementptr %"core::result::Result<u64, u32>", %"core::result::Result<u64, u32>"* %3, i64 0, i32 0
  %5 = load i32, i32* %4, align 8
  %6 = call fastcc zeroext i1 @"_ZN4core6result19Result$LT$T$C$E$GT$5is_ok17hb5269faed3cb7727E"(i32 %5)
  br i1 %6, label %8, label %7

7:                                                ; preds = %12, %8, %1
  ret void

8:                                                ; preds = %1
  %9 = load i128, i128* %2, align 8
  %10 = call fastcc i64 @"_ZN4core6result19Result$LT$T$C$E$GT$6unwrap17he2644901415f1491E"(i128 %9)
  %11 = icmp eq i64 %10, 42
  br i1 %11, label %12, label %7

12:                                               ; preds = %8
  call void @do_not_elim(i64 42)
  br label %7
}
