 asm
    cmpl    $-2147483648, %edi
    jne .LBB0_2
    movl    $-2147483648, %eax
    cmpl    $-1, %esi
    je  .LBB0_3
.LBB0_2:
    movl    %edi, %eax
    cltd
    idivl   %esi
.LBB0_3:
    retq
