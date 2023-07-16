 asm
    .section    .text._ZN25Foo...std..cmp..PartialEq2eq20h0918b2157c9512e2laaE,"ax",@progbits
    .globl  _ZN25Foo...std..cmp..PartialEq2eq20h0918b2157c9512e2laaE
    .align  16, 0x90
    .type   _ZN25Foo...std..cmp..PartialEq2eq20h0918b2157c9512e2laaE,@function
_ZN25Foo...std..cmp..PartialEq2eq20h0918b2157c9512e2laaE:
    .cfi_startproc
    movb    (%rdi), %al
    testb   %al, %al
    je  .LBB0_4
    movzbl  %al, %eax
    cmpl    $1, %eax
    jne .LBB0_2
    movzbl  (%rsi), %ecx
    movb    $1, %al
    cmpl    $1, %ecx
    jne .LBB0_3
    jmp .LBB0_7
.LBB0_4:
    movb    $1, %al
    cmpb    $0, (%rsi)
    jne .LBB0_3
    jmp .LBB0_7
.LBB0_2:
    cmpl    $2, %eax
    jne .LBB0_3
    movb    $1, %al
    movzbl  (%rsi), %ecx
    cmpl    $2, %ecx
    jne .LBB0_3
.LBB0_7:
    retq
.LBB0_3:
    xorl    %eax, %eax
    retq
.Ltmp0:
    .size   _ZN25Foo...std..cmp..PartialEq2eq20h0918b2157c9512e2laaE, .Ltmp0-_ZN25Foo...std..cmp..PartialEq2eq20h0918b2157c9512e2laaE
    .cfi_endproc
