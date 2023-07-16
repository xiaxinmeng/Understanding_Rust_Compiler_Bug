asm
cmp     edi, 128
jae     .LBB0_1
add     dil, -65
mov     al, 1
cmp     dil, 25
ja      .LBB0_1
ret
