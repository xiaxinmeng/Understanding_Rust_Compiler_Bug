asm
example::main:
        sub     rsp, 12
        mov     dword ptr [rsp], 1
        mov     dword ptr [rsp + 4], 2
        mov     dword ptr [rsp + 8], 3
        add     rsp, 12
        ret
