
    7064:       c87f212a        ldxp    x10, x8, [x9] <--(loop trying to complex ld/st pair)--|             
    7068:       f100295f        cmp     x10, #0xa                                             |
    706c:       1a9f97ec        cset    w12, hi  // hi = pmore                                |
    7070:       f100011f        cmp     x8, #0x0                                              |
    7074:       1a9f07ed        cset    w13, ne  // ne = any                                  |
    7078:       1a8d018c        csel    w12, w12, w13, eq  // eq = none                       |
    707c:       7100019f        cmp     w12, #0x0                                             |
    7080:       9a9f110c        csel    x12, x8, xzr, ne  // ne = any                         |
    7084:       9a8b114d        csel    x13, x10, x11, ne  // ne = any                        |
    7088:       c82e312d        stxp    w14, x13, x12, [x9]                                   |
    708c:       35fffece        cbnz    w14, 7064 <ld128b::main+0x14>     --------------------|
