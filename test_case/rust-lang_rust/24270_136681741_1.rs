 asm
_ZN23E...std..cmp..PartialEq2eq20hd7e6941040691683taaE:
    .cfi_startproc
    movl    (%rdi), %eax
    cmpl    (%rsi), %eax
    jne .LBB0_1
    cmpl    $1, %eax
    je  .LBB0_5
    cmpl    $2, %eax
    je  .LBB0_5
    cmpl    $3, %eax
.LBB0_5:
    movl    4(%rdi), %eax
    cmpl    4(%rsi), %eax
    sete    %al
    retq
.LBB0_1:
    xorl    %eax, %eax
    retq
