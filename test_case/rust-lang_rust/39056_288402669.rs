llvm
define i32 @invalid_call(i32, i32) {
  %3 = call { i32, i1 } @llvm.smul.with.overflow.i32(i32 %0, i32 %1)
  %4 = extractvalue { i32, i1 } %3, 0
  %5 = extractvalue { i32, i1 } %3, 1
  br i1 %5, label %overflow, label %no_overflow

no_overflow:
  ret i32 %4

overflow:
  ret i32 0
}

declare { i32, i1 } @llvm.smul.with.overflow.i32(i32, i32)
