
    movl    $32, %eax
    testl   %edi, %edi
    je  .LBB0_2
    bsrl    %edi, %eax
    xorl    $31, %eax
.LBB0_2:
    xorl    $32, %eax
    retq
