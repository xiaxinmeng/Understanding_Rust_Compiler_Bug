
main:
    lui     a0, 1493
    addiw   a0, a0, 413
    slli    a0, a0, 14
    addi    a2, a0, -144
    lui     a0, 11939
    addiw   a0, a0, -679
    slli    a0, a0, 12
    addi    a1, a0, -140
    mv      a0, a2
    .word   0xB5150B
    mv      a0, a2
    .word   0x2B5150B
    mv      a0, a2
    .word   0x4B5150B
    ret
