asm
example::foo_1a:
        xor     r9d, r9d
        movabs  r8, 3777893186295716171
.LBB0_2:
        mov     rax, r9
        mul     r8
        shr     rdx, 10
        imul    rax, rdx, 5000
        mov     rdx, r9
        sub     rdx, rax
        mov     eax, dword ptr [rsi + 4*rdx]
        xor     edx, edx
.LBB0_3:
        mov     ecx, edx
        or      word ptr [rdi + 8*rcx], 1234
        add     edx, eax
        cmp     edx, 4096
        jb      .LBB0_3
        add     r9, 1
        cmp     r9, 10000000
        jne     .LBB0_2
        ret

example::foo_1b:
        xor     ecx, ecx
        movabs  r8, 3777893186295716171
.LBB1_2:
        mov     rax, rcx
        mul     r8
        shr     rdx, 10
        imul    rdx, rdx, 5000
        mov     rax, rcx
        sub     rax, rdx
        xor     edx, edx
.LBB1_3:
        mov     esi, edx
        or      word ptr [rdi + 8*rsi], 1234
        add     edx, dword ptr [rdi + 4*rax + 32768]
        cmp     edx, 4096
        jb      .LBB1_3
        add     rcx, 1
        cmp     rcx, 10000000
        jne     .LBB1_2
        ret
