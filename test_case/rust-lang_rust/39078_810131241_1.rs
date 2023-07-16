asm
foo:
    sub     rsp, 8
    mov     edx, 10
    xor     ecx, ecx
    call    qword ptr [rip + __umodti3@GOTPCREL]
    pop     rcx
    ret
