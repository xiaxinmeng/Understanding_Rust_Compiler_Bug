 asm
checked_:
    add r1, r0, r1
    cmp r1, r0
    movvs   r1, #0
    mov r0, r1
    bx  lr
