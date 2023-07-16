asm
        .text
        .file   "a0-8787f43e282added376259c1adb08b80.rs"
        .section        .text._ZN1a3foo17h55c63ef928c74adbE,"ax",@progbits
        .globl  _ZN1a3foo17h55c63ef928c74adbE
        .p2align        4, 0x90
        .type   _ZN1a3foo17h55c63ef928c74adbE,@function
_ZN1a3foo17h55c63ef928c74adbE:
        pushq   %rbx
        movq    %rdi, %rbx
        movl    $900, %edi
        movl    $1, %esi
        callq   __rust_alloc_zeroed@PLT
        testq   %rax, %rax
        je      .LBB0_1
        movq    %rax, (%rbx)
        movq    $900, 8(%rbx)
        movq    $900, 16(%rbx)
        movq    %rbx, %rax
        popq    %rbx
        retq
.LBB0_1:
        movl    $900, %edi
        movl    $1, %esi
        callq   _ZN5alloc5alloc3oom17h772d6152ef20ca43E@PLT
        ud2
.Lfunc_end0:
        .size   _ZN1a3foo17h55c63ef928c74adbE, .Lfunc_end0-_ZN1a3foo17h55c63ef928c74adbE


        .section        ".note.GNU-stack","",@progbits
