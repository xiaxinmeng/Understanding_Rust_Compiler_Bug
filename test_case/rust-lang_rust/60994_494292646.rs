asm
foo:                                    # @foo
# BB#0:
        xor     rsi, rcx
        xor     rdi, rdx
        or      rdi, rsi
        sete    al
        ret
