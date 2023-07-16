assembly
example::foo:
        xor     ecx, ecx
        cmp     esi, 5
        sete    cl
        mov     al, 2
        sub     al, cl
        mov     byte ptr [rdi], al
        mov     eax, 2
        sub     eax, ecx
        ret
