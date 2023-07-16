
__multi3(0:0:0:0, 0:0:0:A) = 0:0:0:0    ; not sure why it is multiplying 0 with anything, but result is correct nevertheless
__udivti3(0:0:0:0, 0:0:0:A) = 0:0:0:0
__multi3(0:0:0:1, 0:0:0:A) = 0:A:0:14  ; result seems wrong?
__udivti3(0:A:0:14, 0:0:0:A) = 0:0:0:3   ; also seems wrong, even considering `0x14 / 0xA` the result is `2`, not `3`.
// error happens after this point.
