asm
example::f:
        test    edi, edi
        setne   al
        test    esi, esi
        setne   cl
        cmp     al, cl
        je      .LBB0_2
        test    edi, edi
        setne   al
        add     al, al
        add     al, -1
        test    esi, esi
        movzx   ecx, al
        mov     eax, 1
        cmovne  eax, ecx
.LBB0_5:
        cmp     al, -1
        sete    al
        ret
.LBB0_2:
        xor     eax, eax
        test    edi, edi
        je      .LBB0_5
        test    esi, esi
        je      .LBB0_5
        xor     ecx, ecx
        cmp     edi, esi
        setne   cl
        mov     eax, 255
        cmovae  eax, ecx
        cmp     al, -1
        sete    al
        ret
