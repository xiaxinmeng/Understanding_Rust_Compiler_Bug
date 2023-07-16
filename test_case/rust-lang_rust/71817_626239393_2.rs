asm
    .text
    .file   "main.3a1fbbbh-cgu.0"
    .section    .text._ZN4main5write17h1aebe8f6503e0e08E,"ax",@progbits
    .globl  _ZN4main5write17h1aebe8f6503e0e08E .p2align 4, 0x90
    .type   _ZN4main5write17h1aebe8f6503e0e08E,@function
_ZN4main5write17h1aebe8f6503e0e08E:
    .cfi_startproc
    cmpw    $8, %di
    jae .LBB0_1
    movswq  %di, %rax
    leaq    .Lswitch.table._ZN4main5write17h1aebe8f6503e0e08E(%rip), %rcx
    movzwl  (%rcx,%rax,2), %eax
    retq
.LBB0_1:
    addl    $1, %edi
    movl    %edi, %eax
    retq
.Lfunc_end0:
    .size   _ZN4main5write17h1aebe8f6503e0e08E, .Lfunc_end0-_ZN4main5write17h1aebe8f6503e0e08E
    .cfi_endproc

    .type   .Lswitch.table._ZN4main5write17h1aebe8f6503e0e08E,@object
    .section    .rodata.cst16,"aM",@progbits,16
    .p2align    1
.Lswitch.table._ZN4main5write17h1aebe8f6503e0e08E:
    .short  1
    .short  2
    .short  3
    .short  4
    .short  5
    .short  6
    .short  7
    .short  229
    .size   .Lswitch.table._ZN4main5write17h1aebe8f6503e0e08E, 16


    .section    ".note.GNU-stack","",@progbits
