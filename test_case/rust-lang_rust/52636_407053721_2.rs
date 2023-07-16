
   100000ce0:   55                      push   rbp
   100000ce1:   48 89 e5                mov    rbp,rsp
   100000ce4:   41 56                   push   r14
   100000ce6:   53                      push   rbx
   100000ce7:   48 83 e4 e0             and    rsp,0xffffffffffffffe0
   100000ceb:   48 81 ec e0 00 00 00    sub    rsp,0xe0
   100000cf2:   0f 28 05 87 d3 03 00    movaps xmm0,XMMWORD PTR [rip+0x3d387]        # 10003e080 <__ZN53_$LT$$RF$$u27$a$u20$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h743e723bc811d43eE+0x680>
   100000cf9:   0f 29 84 24 90 00 00    movaps XMMWORD PTR [rsp+0x90],xmm0
   100000d00:   00 
   100000d01:   0f 29 84 24 80 00 00    movaps XMMWORD PTR [rsp+0x80],xmm0
   100000d08:   00 
   100000d09:   4c 8d b4 24 a0 00 00    lea    r14,[rsp+0xa0]
   100000d10:   00 
   100000d11:   0f 57 c0                xorps  xmm0,xmm0
   100000d14:   0f 29 84 24 b0 00 00    movaps XMMWORD PTR [rsp+0xb0],xmm0
   100000d1b:   00 
   100000d1c:   0f 29 84 24 a0 00 00    movaps XMMWORD PTR [rsp+0xa0],xmm0
   100000d23:   00 
   100000d24:   0f 28 05 55 d3 03 00    movaps xmm0,XMMWORD PTR [rip+0x3d355]        # 10003e080 <__ZN53_$LT$$RF$$u27$a$u20$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h743e723bc811d43eE+0x680>
   100000d2b:   48 8d 9c 24 c0 00 00    lea    rbx,[rsp+0xc0]
   100000d32:   00 
   100000d33:   48 89 df                mov    rdi,rbx
   100000d36:   0f 28 c8                movaps xmm1,xmm0
   100000d39:   e8 82 ff ff ff          call   100000cc0 <__ZN4core8coresimd3x863avx17_mm256_load_si25617h86383795cdef4461E>
