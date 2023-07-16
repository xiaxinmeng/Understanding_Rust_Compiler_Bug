
GetMyStruct:                    save      %sp, -0x90, %sp
GetMyStruct+4:                  mov       0x1, %i0
GetMyStruct+8:                  stb       %i0, [%fp + 0x7f7]
GetMyStruct+0xc:                ret
GetMyStruct+0x10:               restore   %g0, 0x1, %o0
