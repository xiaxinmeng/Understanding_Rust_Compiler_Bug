 asm
_ZN13trim_in_place20h5d93414c92587a1aeaaE:
    .cfi_startproc
    movq    (%rdi), %rax
    movq    8(%rdi), %rdx
    xorl    %ecx, %ecx
    testq   %rdx, %rdx
    je  .LBB0_5
    xorl    %ecx, %ecx
    .align  16, 0x90
.LBB0_2:
    movzbl  (%rax), %esi
    cmpl    $42, %esi
    jne .LBB0_3
    incq    %rax
    decq    %rdx
    jne .LBB0_2
    jmp .LBB0_5
.LBB0_3:
    movq    %rdx, %rcx
.LBB0_5:
    movq    %rax, (%rdi)
    movq    %rcx, 8(%rdi)
    retq
