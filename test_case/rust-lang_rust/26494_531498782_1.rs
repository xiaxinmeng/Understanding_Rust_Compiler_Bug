asm
example::parameters_by_value:
        lea     rax, [rdi + rsi]
        ret

example::return_by_value:
        mov     eax, 3
        mov     edx, 4
        ret
