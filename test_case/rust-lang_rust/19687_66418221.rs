 asm
    callq   _ZN15n_queens_helper20h5eeb7aeda9e3dcfbFaaE
    leaq    8(%rsp), %rcx
    leaq    16(%rsp), %rdx
    .align  16, 0x90
.LBB2_4:
    movq    %rax, 8(%rsp)
    #APP
    #NO_APP
    #APP
    #NO_APP
    decq    %rbx
    jne .LBB2_4
