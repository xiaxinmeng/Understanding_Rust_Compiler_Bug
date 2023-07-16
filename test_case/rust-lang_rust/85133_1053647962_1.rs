llvm
  %8 = load i32, i32* %7, align 4, !dbg !10, !range !11
  %_2 = zext i32 %8 to i64, !dbg !10
  switch i64 %_2, label %bb2 [
    i64 0, label %bb3
    i64 1, label %bb1
  ], !dbg !12
