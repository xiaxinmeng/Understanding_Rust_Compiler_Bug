
example::fn_cttz:
        test    rdi, rdi
        je      .LBB1_1
        je      .LBB1_3
        bsf     rax, rdi
        ret
.LBB1_1:
        xor     eax, eax
        ret
.LBB1_3:
        mov     eax, 64
        ret
