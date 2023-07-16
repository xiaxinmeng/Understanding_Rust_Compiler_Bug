asm
example::saturating_sub:
        xor     eax, eax
        mov     ecx, edi
        sub     ecx, esi
        setns   al
        add     eax, 2147483647
        sub     edi, esi
        cmovno  eax, edi
        ret
