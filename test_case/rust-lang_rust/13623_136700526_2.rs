 asm
.type   _ZN23E...std..cmp..PartialEq2eq20h8d48eb51e9008df6FaaE,@function
_ZN23E...std..cmp..PartialEq2eq20h8d48eb51e9008df6FaaE:
    .cfi_startproc
    movl    (%rdi), %eax
    cmpl    (%rsi), %eax
    jne .LBB0_1
    decl    %eax
    cmpl    $6, %eax
    jbe .LBB0_3
.LBB0_4:
    movl    4(%rdi), %eax
    cmpl    4(%rsi), %eax
    sete    %al
    retq
.LBB0_1:
    xorl    %eax, %eax
    retq
.LBB0_3:
    leaq    .LJTI0_0(%rip), %rcx
    movslq  (%rcx,%rax,4), %rax
    addq    %rcx, %rax
    jmpq    *%rax
.Lfunc_end0:
    .size   _ZN23E...std..cmp..PartialEq2eq20h8d48eb51e9008df6FaaE, .Lfunc_end0-_ZN23E...std..cmp..PartialEq2eq20h8d48eb51e9008df6FaaE
    .cfi_endproc
    .section    .rodata._ZN23E...std..cmp..PartialEq2eq20h8d48eb51e9008df6FaaE,"a",@progbits
    .align  4
.LJTI0_0:
    .long   .LBB0_4-.LJTI0_0
    .long   .LBB0_4-.LJTI0_0
    .long   .LBB0_4-.LJTI0_0
    .long   .LBB0_4-.LJTI0_0
    .long   .LBB0_4-.LJTI0_0
    .long   .LBB0_4-.LJTI0_0
    .long   .LBB0_4-.LJTI0_0
