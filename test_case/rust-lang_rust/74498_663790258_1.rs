LLVM
loop_hdr:
[...]
  %0 = add nuw i8 %val, 1
[...]
loop_write:
[...]
  %4 = icmp eq i8 %0, 0
  br i1 %4, label %exit, label %loop_hdr
