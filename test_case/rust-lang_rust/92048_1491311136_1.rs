asm
example::midpoint_simple:
        xor     eax, eax
        add     rdi, rsi
        setb    al
        shld    rax, rdi, 63
        ret
