
00000000008870e0 <rand_pool_acquire_entropy>:
  8870e0:       47 00 4c 3c     addis   r2,r12,71
  8870e4:       20 88 42 38     addi    r2,r2,-30688
  ...
  887158:       b1 f3 ff 4b     bl      886508 <rand_pool_add_begin+0x8>
  88715c:       00 00 00 60     nop
  887160:       78 1b 64 7c     mr      r4,r3
  887164:       5c 00 8e 41     beq     cr3,8871c0 <rand_pool_acquire_entropy+0xe0>
  887168:       78 fb e4 7f     mr      r4,r31
  88716c:       95 8e 77 4b     bl      0
