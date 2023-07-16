
(gdb) disas $pc-12,$pc+12
Dump of assembler code from 0xffff0fc0 to 0xffff0fd4:
   0xffff0fc0:  ldrex   r3, [r2]
   0xffff0fc4:  subs    r3, r3, r0
=> 0xffff0fc8:  strexeq r3, r1, [r2]
   0xffff0fcc:  teqeq   r3, #1
   0xffff0fd0:  beq     0xffff0fc0
End of assembler dump.
