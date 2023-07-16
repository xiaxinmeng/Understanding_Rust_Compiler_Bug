asm
div10(unsigned long):                              # @div10(unsigned long)
        mov     rax, rdi
        movabs  rcx, -3689348814741910323
        mul     rcx
        mov     rax, rdx
        shr     rax, 3
        ret
div10(unsigned __int128):                              # @div10(unsigned __int128)
        shrd    rdi, rsi, 1
        shr     rsi
        mov     rcx, rdi
        add     rcx, rsi
        adc     rcx, 0
        movabs  r8, -3689348814741910323
        mov     rax, rcx
        mul     r8
        shr     rdx, 2
        lea     rax, [rdx + 4*rdx]
        sub     rcx, rax
        sub     rdi, rcx
        sbb     rsi, 0
        movabs  rcx, -3689348814741910324
        mov     rax, rdi
        mul     r8
        imul    rcx, rdi
        add     rdx, rcx
        imul    r8, rsi
        add     rdx, r8
        ret
