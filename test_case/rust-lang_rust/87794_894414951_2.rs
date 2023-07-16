asm
 test    dil, dil
 je      .LBB0_1
 movzx   eax, dil
 ret
.LBB0_1:
 xor     eax, eax
 ret
