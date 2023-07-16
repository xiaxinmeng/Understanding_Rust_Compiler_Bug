assembly
example::vec_cast:
        mov     rax, rdi
        mov     rcx, qword ptr [rsi]
        mov     rdx, qword ptr [rsi + 8]
        mov     rsi, qword ptr [rsi + 16]
        test    rsi, rsi
        lea     rsi, [rcx + 8*rsi]
        cmove   rsi, rcx
        sub     rsi, rcx
        sar     rsi, 3
        mov     qword ptr [rdi], rcx
        mov     qword ptr [rdi + 8], rdx
        mov     qword ptr [rdi + 16], rsi
        ret
