asm
example::foo:
        jmp     qword ptr [rip + malloc@GOTPCREL]
