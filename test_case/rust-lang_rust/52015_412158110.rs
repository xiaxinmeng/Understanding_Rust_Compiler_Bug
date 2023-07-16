asm
example::scalar_test:
        stmg    %r14, %r15, 112(%r15)
        aghi    %r15, -160
        j       .LBB47_1
.LBB47_1:
        lhi     %r0, 1
        chi     %r0, 0
        jlh     .LBB47_3
        j       .LBB47_2
.LBB47_2:
        larl    %r2, .Lbyte_str.6
        larl    %r4, .Lbyte_str.5
        lghi    %r3, 14
        brasl   %r14, std::panicking::begin_panic@PLT
.Ltmp29:
        j       .Ltmp29+2
.LBB47_3:
        lmg     %r14, %r15, 272(%r15)
        br      %r14

.Lbyte_str.4:
        .ascii  "/tmp/compiler-explorer-compiler118710-63-1i2e6ad.i6ghg/example.rs"

.Lbyte_str.5:
        .quad   .Lbyte_str.4
        .ascii  "\000\000\000\000\000\000\000A\000\000\000\007\000\000\000\t"

.Lbyte_str.6:
        .ascii  "explicit panic"
