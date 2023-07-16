
Dump of assembler code for function __sync_fetch_and_add_4:
   0x0000f7e0 <+0>:     push    {r4, r5, r6, lr}
   0x0000f7e4 <+4>:     mov     r4, r1
   0x0000f7e8 <+8>:     mov     r2, r0
   0x0000f7ec <+12>:    ldr     r5, [r2]
   0x0000f7f0 <+16>:    ldr     r6, [pc, #24]   ; 0xf810 <__sync_fetch_and_add_4+48>
   0x0000f7f4 <+20>:    add     r1, r5, r4
   0x0000f7f8 <+24>:    mov     r0, r5
   0x0000f7fc <+28>:    blx     r0
   0x0000f800 <+32>:    cmp     r0, #0
   0x0000f804 <+36>:    bne     0xf7ec <__sync_fetch_and_add_4+12>
   0x0000f808 <+40>:    mov     r0, r5
   0x0000f80c <+44>:    pop     {r4, r5, r6, pc}
   0x0000f810 <+48>:                    ; <UNDEFINED> instruction: 0xffff0fc0
