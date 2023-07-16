
        mov     edx, dword ptr [rdi]
        mov     r8d, dword ptr [rsi]
        xor     ecx, ecx
        cmp     edx, r8d
        setne   cl
        mov     eax, 255
        cmovae  eax, ecx
        test    al, al
        je      .LBB0_1
.LBB0_13:
        ret
