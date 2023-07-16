
avx2_bug_hunt`core::coresimd::x86::avx::_mm256_load_si256::h86383795cdef4461:
avx2_bug_hunt[0x100000cc0] <+0>:  push   rbp
avx2_bug_hunt[0x100000cc1] <+1>:  mov    rbp, rsp
avx2_bug_hunt[0x100000cc4] <+4>:  vmovaps ymmword ptr [rdi], ymm0
avx2_bug_hunt[0x100000cc8] <+8>:  pop    rbp
avx2_bug_hunt[0x100000cc9] <+9>:  vzeroupper 
avx2_bug_hunt[0x100000ccc] <+12>: ret    
avx2_bug_hunt[0x100000ccd] <+13>: nop    dword ptr [rax]

avx2_bug_hunt`core::coresimd::x86::avx::_mm256_store_si256::h1b513ee5f14081f3:
avx2_bug_hunt[0x100000cd0] <+0>:  push   rbp
avx2_bug_hunt[0x100000cd1] <+1>:  mov    rbp, rsp
avx2_bug_hunt[0x100000cd4] <+4>:  vmovaps ymmword ptr [rdi], ymm0
avx2_bug_hunt[0x100000cd8] <+8>:  pop    rbp
avx2_bug_hunt[0x100000cd9] <+9>:  vzeroupper 
avx2_bug_hunt[0x100000cdc] <+12>: ret    
avx2_bug_hunt[0x100000cdd] <+13>: nop    dword ptr [rax]
