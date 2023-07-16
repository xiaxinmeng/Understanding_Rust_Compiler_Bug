llvm
loop_block:
   ; yadda yadda
   br %done, label %memcpy_block, label %loop_block

memcpy_block:
    call void @llvm.memcpy(...)
