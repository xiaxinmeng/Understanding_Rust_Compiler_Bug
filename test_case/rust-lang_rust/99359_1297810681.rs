rust
rust_begin_unwind:
        j       .LBB0_1
.LBB0_1:
        j       .LBB0_1

_start:
        li      a0, 71
        lui     t4, 65536
        mv      t5, a0
        sb      t5, 0(t4)
        j       .LBB1_1
.LBB1_1:
        j       .LBB1_1
