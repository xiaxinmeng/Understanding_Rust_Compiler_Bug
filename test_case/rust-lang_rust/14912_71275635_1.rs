
        .text
        .file   "hello.0.rs"
        .section        .text._ZN4main20h9021d633acfe0488eaaE,"ax",@progbits
        .align  16, 0x90
        .type   _ZN4main20h9021d633acfe0488eaaE,@function
_ZN4main20h9021d633acfe0488eaaE:
        .cfi_startproc
        xorl    %eax, %eax
        movq    %rdi, -8(%rsp)
        movq    %rsi, -16(%rsp)
        retq
.Ltmp0:
        .size   _ZN4main20h9021d633acfe0488eaaE, .Ltmp0-_ZN4main20h9021d633acfe0488eaaE
        .cfi_endproc

        .section        .text.main,"ax",@progbits
        .globl  main
        .align  16, 0x90
        .type   main,@function
main:
        .cfi_startproc
        cmpq    %fs:112, %rsp
        ja      .LBB1_0
        movabsq $8, %r10
        movabsq $0, %r11
        callq   __morestack
        retq
.LBB1_0:
        pushq   %rax
.Ltmp1:
        .cfi_def_cfa_offset 16
        callq   _ZN4main20h9021d633acfe0488eaaE
        popq    %rdx
        retq
.Ltmp2:
        .size   main, .Ltmp2-main
        .cfi_endproc


        .section        ".note.GNU-stack","",@progbits

