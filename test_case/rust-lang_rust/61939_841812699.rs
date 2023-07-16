asm
example::noop:
        cmp     qword ptr [rdi + 16], 0
        ret
