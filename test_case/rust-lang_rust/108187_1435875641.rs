asm
.LBB0_5:
        mov     r10, rsi
        and     r10, -32
        lea     rcx, [r10 - 32]
        mov     r8, rcx ; | (2) therefore r8 is 1
        shr     r8, 5   ; |
        inc     r8      ; /
        test    rcx, rcx ; | (1) rcx must be 0
        je      .LBB0_6  ; /
