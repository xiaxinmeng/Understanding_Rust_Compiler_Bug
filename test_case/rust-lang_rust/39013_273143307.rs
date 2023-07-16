
=> 0x5556de48 <+720>:   lw      at,148(sp)
   0x5556de4c <+724>:   ll      v0,0(at)
   0x5556de50 <+728>:   lw      v1,152(sp)
   0x5556de54 <+732>:   sw      v0,92(sp)
   0x5556de58 <+736>:   bne     v0,v1,0x5556de74 <_ZN4core4sync6atomic23atomic_compare_exchange17hd108e2f5e9a6b935E+764>
   0x5556de5c <+740>:   nop
   0x5556de60 <+744>:   lw      at,156(sp)
   0x5556de64 <+748>:   lw      v0,148(sp)
   0x5556de68 <+752>:   sc      at,0(v0)
   0x5556de6c <+756>:   beqz    at,0x5556de48 <_ZN4core4sync6atomic23atomic_compare_exchange17hd108e2f5e9a6b935E+720>
