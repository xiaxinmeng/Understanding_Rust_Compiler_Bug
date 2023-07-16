asm
compare(char const*):
        cmp     WORD PTR [rdi], 12849
        je      .L6
.L2:
        mov     eax, 1
        test    eax, eax
        sete    al
        ret
.L6:
        xor     eax, eax
        cmp     BYTE PTR [rdi+2], 51
        jne     .L2
        test    eax, eax
        sete    al
        ret
