asm
; rustc +01463991259f5ad8ff520b94d9c9c2e72cdf1a98 -Copt-level=3 --crate-type=cdylib fmt.rs
; objdump -Mintel --disassemble=bar ./libfmt.so | rustfilt
    51e0:       41 56                   push   r14
    51e2:       53                      push   rbx
    51e3:       50                      push   rax
    51e4:       49 89 fe                mov    r14,rdi
    51e7:       48 8b 77 10             mov    rsi,QWORD PTR [rdi+0x10]
    51eb:       ba 01 00 00 00          mov    edx,0x1
    51f0:       e8 3b ff ff ff          call   5130 <alloc::raw_vec::RawVec<T,A>::reserve>
    51f5:       49 8b 5e 10             mov    rbx,QWORD PTR [r14+0x10]
    51f9:       49 8b 3e                mov    rdi,QWORD PTR [r14]
    51fc:       48 01 df                add    rdi,rbx
    51ff:       48 8d 15 fa dd 02 00    lea    rdx,[rip+0x2ddfa]        # 33000 <_fini+0x42c>
    5206:       be 01 00 00 00          mov    esi,0x1
    520b:       b9 01 00 00 00          mov    ecx,0x1
    5210:       e8 1b 00 00 00          call   5230 <core::slice::<impl [T]>::copy_from_slice>
    5215:       48 83 c3 01             add    rbx,0x1
    5219:       49 89 5e 10             mov    QWORD PTR [r14+0x10],rbx
    521d:       31 c0                   xor    eax,eax
    521f:       48 83 c4 08             add    rsp,0x8
    5223:       5b                      pop    rbx
    5224:       41 5e                   pop    r14
    5226:       c3                      ret
