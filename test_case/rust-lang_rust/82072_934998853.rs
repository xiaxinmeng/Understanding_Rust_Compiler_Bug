asm
example::fused_addassign_mul_scalar_neon:
        ldr     q0, [x1]
        ushr    v0.16b, v0.16b, #4
        str     q0, [x0]
        ldr     q0, [x1, #16]
        ushr    v0.16b, v0.16b, #4
        str     q0, [x0, #16]
        ret
