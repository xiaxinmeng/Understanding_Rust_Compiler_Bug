asm
right_shift_1(long long __vector(4), long long __vector(4)):               # @right_shift_1(long long __vector(4), long long __vector(4))
        vperm2i128      ymm0, ymm0, ymm1, 33    # ymm0 = ymm0[2,3],ymm1[0,1]
        vpalignr        ymm0, ymm0, ymm1, 1     # ymm0 = ymm1[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15], ymm0[0], ymm1[17,18,19,20,21,22,23,24,25,26,27,28,29,30,31], ymm0[16]
        ret
