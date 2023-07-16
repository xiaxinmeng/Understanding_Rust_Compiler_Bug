asm
example::f:
        xor     eax, eax
        test    esi, esi
        je      .L6
        test    edi, edi
        jne     .L12
        mov     eax, 1
        ret
.L12:
        cmp     edi, esi
        setb    al
.L6:
        ret

example::g:
        cmp     edi, esi
        setb    al
        ret
