asm
example::sqrt_f32:
        flds    4(%esp)
        fsqrt
        retl

example::sqrt_f64:
        fldl    4(%esp)
        fsqrt
        retl
