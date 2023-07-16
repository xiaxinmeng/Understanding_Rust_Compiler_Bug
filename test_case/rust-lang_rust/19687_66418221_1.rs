 asm
    .align  16, 0x90
.LBB1_4:
    xorl    %edi, %edi
    xorl    %esi, %esi
    xorl    %edx, %edx
    callq   _ZN15n_queens_helper20h7ab2ef9deb11893eFaaE
    movq    %rax, 8(%rsp)
    #APP
    #NO_APP
    #APP
    #NO_APP
    decq    %rbx
    jne .LBB1_4
