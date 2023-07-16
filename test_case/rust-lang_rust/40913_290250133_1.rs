llvm
  %4 = load i64 (i8*, i64)*, i64 (i8*, i64)** %3, !dbg !254
  %5 = bitcast i64 (i8*, i64)* %4 to i8*, !dbg !254
  %6 = call zeroext i1 @"_ZN4core3ptr31_$LT$impl$u20$$BP$mut$u20$T$GT$7is_null17hdd9ae6e9ddd6f4bcE"(i8* %5), !dbg !254
