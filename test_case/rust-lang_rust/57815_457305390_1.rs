asm
example::unlkly:
        cmpl    %esi, %edi
        jne     .LBB2_1
        retq
.LBB2_1:
       # [doing stuff]
        callq   example::moep
        ud2
