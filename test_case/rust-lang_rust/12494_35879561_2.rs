 llvm
  %283 = getelementptr inbounds [4 x i64]* %282, i64 0, i64 0
  %284 = load i64* %283, align 8
  %285 = trunc i64 %284 to i8
  %286 = lshr i64 %284, 32
  %287 = trunc i64 %286 to i32
  switch i8 %285, label %match_else42 [
