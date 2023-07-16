asm
foo1(unsigned int, unsigned long):                              # @foo1(unsigned int, unsigned long)
        cmp     rsi, 5
        ja      .LBB0_1
        mov     eax, edi
        xor     edx, edx
        div     dword ptr [4*rsi + foo1(unsigned int, unsigned long)::PS]
        mov     eax, edx
        movabs  rcx, 4294967296
        or      rax, rcx
        ret
.LBB0_1:
        xor     eax, eax
        xor     ecx, ecx
        or      rax, rcx
        ret
foo1(unsigned int, unsigned long)::PS:
        .long   2                       # 0x2
        .long   3                       # 0x3
        .long   5                       # 0x5
        .long   7                       # 0x7
        .long   11                      # 0xb
        .long   13                      # 0xd
