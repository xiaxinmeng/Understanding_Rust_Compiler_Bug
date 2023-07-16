asm
        .section        .text.add1_match,"ax",@progbits
        .globl  add1_match
        .p2align        4, 0x90
        .type   add1_match,@function
add1_match:
        .cfi_startproc
        movl    (%rdi), %ecx
        movl    $99992, %eax
        cmpl    (%rsi), %ecx
        jne     .LBB7_3
        movl    %ecx, %eax
        leaq    .LJTI7_0(%rip), %rcx
        movslq  (%rcx,%rax,4), %rax
        addq    %rcx, %rax
        jmpq    *%rax
.LBB7_2:
        movl    4(%rsi), %eax
        addl    4(%rdi), %eax
.LBB7_3:
        retq
.Lfunc_end7:
        .size   add1_match, .Lfunc_end7-add1_match
        .cfi_endproc
        .section        .rodata.add1_match,"a",@progbits
        .p2align        2
.LJTI7_0:
        .long   .LBB7_2-.LJTI7_0
        .long   .LBB7_2-.LJTI7_0
        .long   .LBB7_2-.LJTI7_0
        .long   .LBB7_2-.LJTI7_0
