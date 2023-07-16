asm
foo:
    mov     r8, qword ptr [example::BAR]

example::BAR:
    .zero   8
