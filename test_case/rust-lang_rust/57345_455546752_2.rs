
 230:   78 f3 c4 7f     mr      r4,r30
 234:   78 eb a3 7f     mr      r3,r29
 238:   01 00 00 48     bl      238 <rand_pool_acquire_entropy+0x88>
                        238: R_PPC64_REL24      rand_pool_add_begin
 23c:   00 00 00 60     nop
 240:   78 f3 c4 7f     mr      r4,r30
 244:   01 00 00 48     bl      244 <rand_pool_acquire_entropy+0x94>
                        244: R_PPC64_REL24      getentropy
 248:   00 00 00 60     nop
