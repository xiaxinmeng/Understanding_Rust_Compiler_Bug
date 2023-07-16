asm
atomictest::main:
 sub     sp, sp, #16
 mov     w9, #3
 mov     x8, sp
 stp     x9, xzr, [sp]
 mov     w9, #7
.LBB5_1:
 ldxp    x10, x11, [x8]
 cmp     x10, #7
 cset    w12, hi
 cmp     x11, #0
 cset    w13, ne
 csel    w12, w12, w13, eq
 cmp     w12, #0
 csel    x11, x11, xzr, ne
 csel    x10, x10, x9, ne
 stxp    w12, x10, x11, [x8]
 cbnz    w12, .LBB5_1
 add     sp, sp, #16
 ret
