asm
.LBB0_6: ; only pred is .LBB0_5
        xor     ecx, ecx
        test    r8b, 1   ; | (3) always taken
        jne     .LBB0_11 ; /
        jmp     .LBB0_12 ; (4) dead
