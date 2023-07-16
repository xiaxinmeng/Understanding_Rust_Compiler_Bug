
00000000000071ec <_ZN4main20h74a595d6a5b2fbb6faaE>:
    71ec:       a9be6ffc        stp     x28, x27, [sp,#-32]!
    71f0:       a9017bfd        stp     x29, x30, [sp,#16]
    71f4:       910043fd        add     x29, sp, #0x10
    71f8:       d104c3ff        sub     sp, sp, #0x130
    71fc:       321a0fe8        orr     w8, wzr, #0x3c0
    7200:       2a0803e9        mov     w9, w8
    7204:       f81e83a9        str     x9, [x29,#-24]
    7208:       94000047        bl      7324 <_ZN3num7i32.Int9min_value20hb9150aaae7f1b75agpbE>
    720c:       b81e43a0        str     w0, [x29,#-28]
    7210:       b89e43a9        ldrsw   x9, [x29,#-28]
    7214:       f85e83aa        ldr     x10, [x29,#-24]
    7218:       eb0a0129        subs    x9, x9, x10
