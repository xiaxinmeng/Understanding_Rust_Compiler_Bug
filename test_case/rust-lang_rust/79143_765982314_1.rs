asm
foo1:
    test    edi, edi
    je      .LBB0_2
    bsf     eax, edi
    ret
.LBB0_2:
    mov     eax, 32
    ret

foo2:
    bsf     eax, edi
    ret
