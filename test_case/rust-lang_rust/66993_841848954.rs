asm
example::rem:
        movsx   eax, dil
        imul    eax, eax, -109
        shr     eax, 8
        add     al, dil
        mov     ecx, eax
        shr     cl, 7
        sar     al, 2
        add     al, cl
        movzx   eax, al
        lea     ecx, [8*rax]
        sub     ecx, eax
        sub     dil, cl
        lea     eax, [rdi + 7]
        test    dil, dil
        movzx   ecx, dil
        movzx   eax, al
        cmovns  eax, ecx
        ret
