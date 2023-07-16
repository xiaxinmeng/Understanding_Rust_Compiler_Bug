 asm
.LBB0_64:
    movq    %rbx, %rdi
    callq   black@PLT
.LBB0_65:
    movzbl  (%rbx), %eax
    cmpl    $62, %eax
    je  .LBB0_64
    decq    %rbx
    jmp .LBB0_65
