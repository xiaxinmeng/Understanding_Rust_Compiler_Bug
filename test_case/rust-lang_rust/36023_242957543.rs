 llvm
bb2.i:                                            ; preds = %bb3
  %4 = bitcast [3 x i64]* %3 to %"3.std::string::String"*
  %5 = getelementptr inbounds %"3.std::string::String", %"3.std::string::String"* %4, i64 0, i32 0, i32 0, i32 0, i32 0, i32 0
  %6 = load i8*, i8** %5, align 8, !alias.scope !2, !nonnull !9
