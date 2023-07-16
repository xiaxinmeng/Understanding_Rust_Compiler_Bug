asm
compare(char const*):                          # @compare(char const*)
        push    rax
        mov     esi, .L.str
        mov     edx, 3
        call    memcmp
        test    eax, eax
        sete    al
        pop     rcx
        ret

.L.str:
        .asciz  "123"
