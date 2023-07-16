rust
example::foo:
        test    rsi, rsi
        je      .LBB0_1
        xor     eax, eax
        xor     edx, edx
.LBB0_3:
        cmp     byte ptr [rdi + rdx], 0
        je      .LBB0_4
        add     rdx, 1
        cmp     rsi, rdx
        jne     .LBB0_3
        mov     rdx, rsi
        ret
.LBB0_1:
        xor     edx, edx
        xor     eax, eax
        ret
.LBB0_4:
        mov     eax, 1
        ret
