asm
times_ten(int):                          # @times_ten(int)
        add     edi, edi
        lea     eax, [rdi + 4*rdi]
        ret
