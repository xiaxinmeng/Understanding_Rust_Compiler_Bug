asm
example::foo:
        xor     eax, eax
        xor     ecx, ecx
        mov     rax, rbx
        cpuid
        xchg    rax, rbx
        ret
