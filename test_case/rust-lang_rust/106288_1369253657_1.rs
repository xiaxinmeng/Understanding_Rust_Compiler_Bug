nasm
example::fold_ref:
        test    rsi, rsi
        lea     rax, [rdi + 4*rsi - 4]
        cmove   rax, rsi
        ret
