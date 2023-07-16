asm
example::return_result_ok:
        xor     eax, eax
        mov     edx, 7
        ret

example::return_result_err:
        mov     eax, 1
        mov     edx, 7
        ret
