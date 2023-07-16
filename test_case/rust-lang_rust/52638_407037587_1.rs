
_Z11GetMyStructv:               save      %sp, -0xc0, %sp
_Z11GetMyStructv+4:             mov       0x1, %g1
_Z11GetMyStructv+8:             stb       %g1, [%fp + 0x7fe]
_Z11GetMyStructv+0xc:           ldub      [%fp + 0x7fe], %g1
_Z11GetMyStructv+0x10:          and       %g1, 0xff, %g1
_Z11GetMyStructv+0x14:          sllx      %g1, 0x38, %g1
_Z11GetMyStructv+0x18:          mov       %g1, %i0
_Z11GetMyStructv+0x1c:          return    %i7 + 0x8
_Z11GetMyStructv+0x20:          nop

