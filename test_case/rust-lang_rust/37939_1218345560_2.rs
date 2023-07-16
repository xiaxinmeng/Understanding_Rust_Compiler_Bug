nasm
example::result_nop_match:
        xor     ecx, ecx
        test    edi, edi
        setne   cl
        movabs  rax, -4294967296
        and     rax, rdi
        or      rax, rcx
        ret
