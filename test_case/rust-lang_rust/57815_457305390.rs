asm
example::unlkly:
        pushq   %rax
        cmpl    %esi, %edi
        jne     .LBB2_1
        popq    %rcx
        retq
.LBB2_1:
        callq   example::moep
        ud2
