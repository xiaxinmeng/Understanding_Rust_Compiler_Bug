asm
example::foo2:
        sub     rsp, 104
        mov     dword ptr [rsp], edi
        mov     dword ptr [rsp + 4], esi
        cmp     edi, esi
        jne     .LBB9_1
        mov     al, 1
        add     rsp, 104
        ret
