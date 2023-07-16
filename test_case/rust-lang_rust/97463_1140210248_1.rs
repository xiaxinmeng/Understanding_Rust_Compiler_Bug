
        bl      rand
        mov     w21, w0
*       bl      rand
        and     w20, w0, 65535
*       mov     w19, w0
        bl      rand
        and     w3, w0, 65535
        mov     w2, w20
        and     w1, w21, 65535
        adrp    x0, .LC0
        add     x0, x0, :lo12:.LC0
        bl      printf
        mov     w1, w20
        adrp    x0, .LC1
        add     x0, x0, :lo12:.LC1
        bl      printf
*       mov     w0, w19
