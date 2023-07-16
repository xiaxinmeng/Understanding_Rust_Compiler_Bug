
00008554 <main>:
    8554:       e92d4800        push    {fp, lr}
    8558:       e59f000c        ldr     r0, [pc, #12]   ; 856c <main+0x18>
    855c:       e3a01000        mov     r1, #0
    8560:       e3a02000        mov     r2, #0
    8564:       eb0002ef        bl      9128 <__sync_val_compare_and_swap_4>
    8568:       e8bd8800        pop     {fp, pc}
    856c:       0000aadc        .word   0x0000aadc
