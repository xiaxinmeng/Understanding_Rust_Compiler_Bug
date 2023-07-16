asm
example::foo:
        push    esi
        xor     eax, eax
        xor     ecx, ecx
        mov     esi, ebx
        cpuid
        xchg    esi, ebx
        pop     esi
        ret
