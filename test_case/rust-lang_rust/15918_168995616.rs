 rust
.LBB0_1:
    movq    -16(%rsp), %rax
    addq    $1, %rax
    cmpq    $-1, %rax
    movq    %rax, -16(%rsp)
    jb  .LBB0_1
