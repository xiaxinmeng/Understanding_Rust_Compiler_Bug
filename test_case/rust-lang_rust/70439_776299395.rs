
example::test:
        mov     rax, rdi
        bswap   rsi
        bswap   rdx
        mov     qword ptr [rdi], rsi
        mov     qword ptr [rdi + 8], rdx
        ret
