
before:
        mov     dword ptr [rdi], ecx
        mov     dword ptr [rdi + 4], r8d
        mov     dword ptr [rdi + 8], edx
        ret

after:
        movd    edx, xmm0
        movd    ecx, xmm1
        shl     rcx, 32
        or      rax, rcx
        ret
