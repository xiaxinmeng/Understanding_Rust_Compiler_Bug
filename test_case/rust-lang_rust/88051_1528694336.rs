
example::a:
        xor     ecx, ecx
        cmp     edi, esi
        sete    cl
        xor     ecx, 3
        cmp     edi, esi
        mov     eax, 1
        cmovge  eax, ecx
        ret

example::b:
        xor     eax, eax
        cmp     edi, esi
        setg    al
        lea     ecx, [rax + rax + 1]
        mov     eax, 2
        cmovne  eax, ecx
        ret
