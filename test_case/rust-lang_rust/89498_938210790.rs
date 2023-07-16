llvm
*** IR Dump Before Module Verifier (verify) ***
; Function Attrs: nonlazybind uwtable
define { i64, i8 } @llvm_test() unnamed_addr #0 !dbg !6 {
start:
  %0 = alloca { i64, i8 }, align 4
  %1 = bitcast { i64, i8 }* %0 to i64*, !dbg !18
  store i64 24234324, i64* %1, align 4, !dbg !18
  %2 = getelementptr inbounds { i64, i8 }, { i64, i8 }* %0, i32 0, i32 1, !dbg !18
  store i8 0, i8* %2, align 4, !dbg !18
  %3 = getelementptr inbounds { i64, i8 }, { i64, i8 }* %0, i32 0, i32 0, !dbg !19
  %4 = load i64, i64* %3, align 4, !dbg !19
  %5 = getelementptr inbounds { i64, i8 }, { i64, i8 }* %0, i32 0, i32 1, !dbg !19
  %6 = load i8, i8* %5, align 4, !dbg !19, !range !20
  %7 = trunc i8 %6 to i1, !dbg !19
  %8 = zext i1 %7 to i8, !dbg !19
  %9 = insertvalue { i64, i8 } undef, i64 %4, 0, !dbg !19
  %10 = insertvalue { i64, i8 } %9, i8 %8, 1, !dbg !19
  ret { i64, i8 } %10, !dbg !19
}
