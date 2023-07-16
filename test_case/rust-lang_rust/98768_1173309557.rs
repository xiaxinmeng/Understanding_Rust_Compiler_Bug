asm
rust_begin_unwind:
        hint    #34
        b       .LBB0_1
.LBB0_1:
        b       .LBB0_1

_hlt:
        hint    #34
        hlt     #0x1
        brk     #0x1

_start:
        hint    #34
        str     x30, [sp, #-16]!
        bl      _hlt
        brk     #0x1
