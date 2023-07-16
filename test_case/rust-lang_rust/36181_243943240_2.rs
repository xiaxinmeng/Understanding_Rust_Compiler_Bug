 llvm
  %2 = icmp eq i32 %0, %1
  %3 = call zeroext i1 @llvm.expect.i1(i1 zeroext %2, true)
  br label %bb1
