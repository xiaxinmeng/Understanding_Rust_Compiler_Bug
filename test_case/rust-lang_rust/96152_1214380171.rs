asm
example::old_style:
        mov     rax, rdi
        or      rax, 1
        ret

example::cheri_compat:
        mov     eax, edi
        not     eax
        and     eax, 1
        add     rax, rdi
        ret

example::fast:
        mov     rax, rdi
        or      rax, 1
        ret
