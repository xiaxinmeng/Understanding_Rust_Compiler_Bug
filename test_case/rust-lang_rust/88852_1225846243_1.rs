
example::from_be_array_intrinsic:
        srliw   a1, a0, 8
        lui     a2, 16
        addiw   a2, a2, -256
        and     a1, a1, a2
        srliw   a2, a0, 24
        or      a1, a1, a2
        slli    a2, a0, 8
        lui     a3, 4080
        and     a2, a2, a3
        slliw   a0, a0, 24
        or      a0, a0, a2
        or      a0, a0, a1
        ret
