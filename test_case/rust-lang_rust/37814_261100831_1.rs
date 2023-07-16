 asm
00000000 <foo>:
   0:   e24dd008        sub     sp, sp, #8
   4:   ec410b10        vmov    d0, r0, r1
   8:   ed8d0b00        vstr    d0, [sp]
   c:   eaffffff        b       10 <foo+0x10>
  10:   ed9d0b00        vldr    d0, [sp]
  14:   ec510b10        vmov    r0, r1, d0
  18:   e28dd008        add     sp, sp, #8
  1c:   e12fff1e        bx      lr

Disassembly of section .text.bar:

00000000 <bar>:
   0:   e24dd008        sub     sp, sp, #8
   4:   ed8d0b00        vstr    d0, [sp]
   8:   eaffffff        b       c <bar+0xc>
   c:   ed9d0b00        vldr    d0, [sp]
  10:   e28dd008        add     sp, sp, #8
  14:   e12fff1e        bx      lr
