 asm
.LBB0_2:
    testq   %rdx, %rdx
    je  .LBB0_4
    addl    (%rdx), %eax
    addq    $4, %rdx
    addq    $-4, %rcx
    jne .LBB0_2
