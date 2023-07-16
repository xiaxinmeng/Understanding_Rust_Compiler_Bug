rust
foo:
        mov     r11, rsi
        shr     r11
        mov     eax, 1
        sub     rsi, r11
        je      .LBB0_1
        mov     r8d, edx
        lea     r10, [rdi + 4*r11]
        xor     edx, edx
        mov     r9d, 255
        jmp     .LBB0_3
.LBB0_6:
        mov     rcx, r11
        shr     rcx
        lea     r10, [rdi + 4*rcx]
        sub     r11, rcx
        mov     rsi, r11
        mov     r11, rcx
        je      .LBB0_8
.LBB0_3:
        xor     ecx, ecx
        cmp     dword ptr [r10], r8d
        setne   cl
        cmova   ecx, r9d
        cmp     cl, 1
        je      .LBB0_6
        cmp     cl, -1
        jne     .LBB0_7
        add     r10, 4
        dec     rsi
        add     rdx, r11
        inc     rdx
        mov     r11, rsi
        mov     rdi, r10
        jmp     .LBB0_6
.LBB0_1:
        xor     edx, edx
        ret
.LBB0_7:
        add     rdx, r11
        xor     eax, eax
.LBB0_8:
        ret
