
Dump of assembler code for function __sync_fetch_and_add_4:
   0x0000fc18 <+0>:     push    {r4, r5, r6, r7, r8, lr}
   0x0000fc1c <+4>:     mov     r5, r0
   0x0000fc20 <+8>:     mov     r7, r1
   0x0000fc24 <+12>:    ldr     r6, [pc, #40]   ; 0xfc54 <__sync_fetch_and_add_4+60>
   0x0000fc28 <+16>:    ldr     r4, [r5]
   0x0000fc2c <+20>:    mov     r2, r5
   0x0000fc30 <+24>:    add     r1, r4, r7
   0x0000fc34 <+28>:    mov     r0, r4
   0x0000fc38 <+32>:    mov     lr, pc
   0x0000fc3c <+36>:    bx      r6
   0x0000fc40 <+40>:    cmp     r0, #0
   0x0000fc44 <+44>:    bne     0xfc28 <__sync_fetch_and_add_4+16>
   0x0000fc48 <+48>:    mov     r0, r4
   0x0000fc4c <+52>:    pop     {r4, r5, r6, r7, r8, lr}
   0x0000fc50 <+56>:    bx      lr
   0x0000fc54 <+60>:                    ; <UNDEFINED> instruction: 0xffff0fc0
