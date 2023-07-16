asm
example::saturating_sub:
        xor     eax, eax
        cmp     edi, esi
        setl    al
        add     eax, 2147483647
        sub     edi, esi
        cmovno  eax, edi
        ret
