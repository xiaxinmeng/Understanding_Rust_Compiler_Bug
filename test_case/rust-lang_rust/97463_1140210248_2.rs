
        bl      c_read_value
        adrp    x22, .L__unnamed_1
        adrp    x20, <&T as core::fmt::Display>::fmt
        mov     x8, sp
        add     x22, x22, :lo12:.L__unnamed_1
        add     x20, x20, :lo12:<&T as core::fmt::Display>::fmt
        adrp    x23, :got:_ZN4core3fmt3num3imp52_$LT$impl$u20$core..fmt..Display$u20$for$u20$u32$GT$3fmt17h78bd006cfbffcd0fE
        strh    w0, [sp]
