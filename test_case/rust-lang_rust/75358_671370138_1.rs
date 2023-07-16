asm
; rustc +13290e83a6e20f3b408d177a9d64d8cf98fe4615 -Copt-level=3 --crate-type=cdylib fmt.rs
; objdump -Mintel --disassemble=bar ./libfmt.so | rustfilt
    51e0:       48 83 ec 38             sub    rsp,0x38
    51e4:       48 89 3c 24             mov    QWORD PTR [rsp],rdi
    51e8:       48 8d 05 69 a8 03 00    lea    rax,[rip+0x3a869]        # 3fa58 <__do_global_dtors_aux_fini_array_entry+0x8>
    51ef:       48 89 44 24 08          mov    QWORD PTR [rsp+0x8],rax
    51f4:       48 c7 44 24 10 01 00    mov    QWORD PTR [rsp+0x10],0x1
    51fb:       00 00
    51fd:       48 c7 44 24 18 00 00    mov    QWORD PTR [rsp+0x18],0x0
    5204:       00 00
    5206:       48 8d 05 fb dd 02 00    lea    rax,[rip+0x2ddfb]        # 33008 <_fini+0x454>
    520d:       48 89 44 24 28          mov    QWORD PTR [rsp+0x28],rax
    5212:       48 c7 44 24 30 00 00    mov    QWORD PTR [rsp+0x30],0x0
    5219:       00 00
    521b:       48 8d 35 9e a8 03 00    lea    rsi,[rip+0x3a89e]        # 3fac0 <anon.4574bf75806fea7c4a1d662dd29266f6.0.llvm.133549432485651853>
    5222:       48 89 e7                mov    rdi,rsp
    5225:       48 8d 54 24 08          lea    rdx,[rsp+0x8]
    522a:       ff 15 b0 c8 03 00       call   QWORD PTR [rip+0x3c8b0]        # 41ae0 <_GLOBAL_OFFSET_TABLE_+0x70>
    5230:       48 83 c4 38             add    rsp,0x38
    5234:       c3                      ret

