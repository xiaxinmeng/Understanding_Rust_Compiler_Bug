llvm
catch.i:                                          ; preds = %.noexc
  %120 = phi i64* [ getelementptr inbounds ([24 x i64], [24 x i64]* @__llvm_gcov_ctr.27, i64 0, i64 11), %.noexc ], !dbg !2861
  %121 = landingpad { i8*, i32 }
          catch i8* null, !dbg !2861
  %122 = load i64, i64* %120, !dbg !2861
  %123 = add i64 %122, 1, !dbg !2861
  store i64 %123, i64* %120, !dbg !2861