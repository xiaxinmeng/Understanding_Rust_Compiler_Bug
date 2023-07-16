asm
example::saturating_sub:
        xor     eax, eax
        sub     edi, esi
        setl    al
        add     eax, 2147483647
        cmovno  eax, edi
        ret
