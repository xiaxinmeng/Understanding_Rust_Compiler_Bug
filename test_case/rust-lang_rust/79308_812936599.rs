asm
example::vec_cast:
        mov     rax, rdi
        mov     rcx, qword ptr [rsi]
        movups  xmm0, xmmword ptr [rsi + 8]
        mov     qword ptr [rdi], rcx
        movups  xmmword ptr [rdi + 8], xmm0
        ret
