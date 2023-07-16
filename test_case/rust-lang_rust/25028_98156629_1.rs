 asm
    .section    .text._ZN12remove_range20h1888d273e1a01667eaaE,"ax",@progbits
    .globl  _ZN12remove_range20h1888d273e1a01667eaaE
    .align  16, 0x90
    .type   _ZN12remove_range20h1888d273e1a01667eaaE,@function
_ZN12remove_range20h1888d273e1a01667eaaE:
    .cfi_startproc
    cmpq    %fs:112, %rsp
    ja  .LBB0_2
    movabsq $24, %r10
    movabsq $0, %r11
    callq   __morestack
    retq
.LBB0_2:
    pushq   %r15
.Ltmp0:
    .cfi_def_cfa_offset 16
    pushq   %r14
.Ltmp1:
    .cfi_def_cfa_offset 24
    pushq   %rbx
.Ltmp2:
    .cfi_def_cfa_offset 32
.Ltmp3:
    .cfi_offset %rbx, -32
.Ltmp4:
    .cfi_offset %r14, -24
.Ltmp5:
    .cfi_offset %r15, -16
    movq    %rsi, %rbx
    movq    %rdi, %r14
    cmpq    %rdx, %rbx
    ja  .LBB0_6
    movq    8(%r14), %r15
    subq    %rdx, %r15
    jb  .LBB0_6
    movq    %rbx, 8(%r14)
    je  .LBB0_6
    movq    (%r14), %rdi
    addq    %rdi, %rdx
    addq    %rbx, %rdi
    movq    %rdx, %rsi
    movq    %r15, %rdx
    callq   memmove@PLT
    addq    %rbx, %r15
    movq    %r15, 8(%r14)
.LBB0_6:
    popq    %rbx
    popq    %r14
    popq    %r15
    retq
.Ltmp6:
    .size   _ZN12remove_range20h1888d273e1a01667eaaE, .Ltmp6-_ZN12remove_range20h1888d273e1a01667eaaE
    .cfi_endproc
