llvm
define i64 @_ZN7example14to_array_outer17h4eae74749e7c0538E(<8 x i8>* %x) unnamed_addr #0 !dbg !5 {
  %0 = alloca [8 x i8], align 1
  %y = load <8 x i8>, <8 x i8>* %x, align 8, !dbg !10
  %1 = bitcast <8 x i8> %y to [8 x i8], !dbg !11
  store [8 x i8] %1, [8 x i8]* %0, align 1, !dbg !11
  %2 = bitcast [8 x i8]* %0 to i64*, !dbg !13
  %3 = load i64, i64* %2, align 1, !dbg !13
  ret i64 %3, !dbg !13
}
