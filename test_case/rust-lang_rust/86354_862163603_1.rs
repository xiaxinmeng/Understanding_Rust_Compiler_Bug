asm
example::foo:
        mov     r10, rsi
        shr     r10
        mov     eax, 1
        sub     rsi, r10
        je      .LBB0_8
        mov     r8d, edx
        lea     r9, [rdi + 4*r10]
        xor     edx, edx
        jmp     .LBB0_4
.LBB0_2:
        add     r9, 4
        add     rsi, -1
        add     rdx, r10
        add     rdx, 1
        mov     r10, rsi
        mov     rdi, r9
.LBB0_3:
        mov     rcx, r10
        shr     rcx
        lea     r9, [rdi + 4*rcx]
        sub     r10, rcx
        mov     rsi, r10
        mov     r10, rcx
        je      .LBB0_7
.LBB0_4:
        cmp     dword ptr [r9], r8d
        ja      .LBB0_2
        jne     .LBB0_3
        add     rdx, r10
        xor     eax, eax
.LBB0_7:
        ret
.LBB0_8:
        xor     edx, edx
        ret
