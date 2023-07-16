asm
example::h:
        test    edi, edi
        setne   al
        test    esi, esi
        setne   cl
        xor     cl, al
        je      .LBB0_2
        xor     eax, eax
        ret
.LBB0_2:
        test    esi, esi
        sete    al
        test    edi, edi
        sete    cl
        or      cl, al
        cmp     edi, esi
        sete    al
        or      al, cl
        ret
