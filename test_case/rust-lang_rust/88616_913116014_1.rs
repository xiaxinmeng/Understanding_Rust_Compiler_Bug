asm
example::qux:
        mov     rax, rdi
        mov     rcx, qword ptr [rsi]
        mov     rdx, qword ptr [rsi + 24]
        mov     qword ptr [rdi + 24], rdx
        movups  xmm0, xmmword ptr [rsi + 8]
        movups  xmmword ptr [rdi + 8], xmm0
        mov     qword ptr [rdi], rcx
        ret
