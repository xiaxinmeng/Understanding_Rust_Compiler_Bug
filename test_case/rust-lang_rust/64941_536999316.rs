asm
core::num::<impl u32>::max_value:
        mov     eax, 4294967295
        ret

example::bar:
        push    rax
        call    core::num::<impl u32>::max_value
        mov     dword ptr [rsp + 4], eax
        mov     eax, dword ptr [rsp + 4]
        pop     rcx
        ret
