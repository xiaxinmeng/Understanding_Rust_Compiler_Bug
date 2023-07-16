
 $ gdb -q --args cargo build
Reading symbols from cargo...done.
(gdb) r
Starting program: /home/pi/.cargo/bin/cargo build
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/arm-linux-gnueabihf/libthread_db.so.1".
process 1600 is executing new program: /home/pi/.rustup/toolchains/nightly-2019-07-20-arm-unknown-linux-gnueabihf/bin/cargo
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/arm-linux-gnueabihf/libthread_db.so.1".
    Updating crates.io index
[New Thread 0xb6d7b2f0 (LWP 1615)]
[Thread 0xb6d7b2f0 (LWP 1615) exited]

Thread 1 "cargo" received signal SIGILL, Illegal instruction.
0x00566c54 in cargo::util::progress::State::tick ()
(gdb) bt full
#0  0x00566c54 in cargo::util::progress::State::tick ()
No symbol table info available.
#1  0xbeff856c in ?? ()
No symbol table info available.
Backtrace stopped: previous frame identical to this frame (corrupt stack?)
   lqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqk
  >x0x566c54 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+124>  vcvt.f64.u32   d16, s0                                                                                                                                                                                x
   x0x566c58 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+128>  vmov   s0, r6                                                                                                                                                                                         x
   x0x566c5c <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+132>  vcvt.f64.u32   d17, s0                                                                                                                                                                                x
   x0x566c60 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+136>  ldr    r2, [r4, #140]  ; 0x8c                                                                                                                                                                         x
   x0x566c64 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+140>  add    r2, r2, #1                                                                                                                                                                                     x
   x0x566c68 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+144>  str    r2, [r4, #140]  ; 0x8c                                                                                                                                                                         x
   x0x566c6c <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+148>  stmib  sp, {r6, r7}                                                                                                                                                                                   x
   x0x566c70 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+152>  vdiv.f64       d16, d17, d16                                                                                                                                                                          x
   x0x566c74 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+156>  vmov   r0, r1, d16                                                                                                                                                                                    x
   x0x566c78 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+160>  vldr   d17, [pc, #1008]        ; 0x567070 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+1176>                                                                      x
   x0x566c7c <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+164>  bic    r1, r1, #-2147483648    ; 0x80000000                                                                                                                                                           x
   x0x566c80 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+168>  vmov   d18, r0, r1                                                                                                                                                                                    x
   x0x566c84 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+172>  ldrb   r0, [r9, #12]                                                                                                                                                                                  x
   x0x566c88 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+176>  vcmpe.f64      d18, d17                                                                                                                                                                               x
   x0x566c8c <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+180>  vmrs   APSR_nzcv, fpscr                                                                                                                                                                               x
   x0x566c90 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+184>  vmovmi.f64     d8, d16                                                                                                                                                                                x
   x0x566c94 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+188>  cmp    r0, #1                                                                                                                                                                                         x
   x0x566c98 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+192>  bne    0x566cec <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+276>                                                                                                 x
   x0x566c9c <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+196>  mov    r1, #2                                                                                                                                                                                         x
   x0x566ca0 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+200>  add    r2, sp, #56     ; 0x38                                                                                                                                                                         x
   x0x566ca4 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+204>  ldr    r0, [pc, #980]  ; 0x567080 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+1192>                                                                              x
   mqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqj
multi-thre Thread 0xb6ff0010 ( In: _ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568                                                                                                                                                                      L??   PC: 0x566c54
(gdb) disassemble
Dump of assembler code for function _ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568:
   0x00566bd8 <+0>:     push    {r4, r5, r6, r7, r8, r9, r10, r11, lr}
   0x00566bdc <+4>:     sub     sp, sp, #4
   0x00566be0 <+8>:     vpush   {d8}
   0x00566be4 <+12>:    sub     sp, sp, #72     ; 0x48
   0x00566be8 <+16>:    mov     r9, r0
   0x00566bec <+20>:    ldrb    r0, [r0, #52]   ; 0x34
   0x00566bf0 <+24>:    mov     r4, #0
   0x00566bf4 <+28>:    mov     r5, #0
   0x00566bf8 <+32>:    cmp     r0, #0
   0x00566bfc <+36>:    bne     0x566fd0 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+1016>
   0x00566c00 <+40>:    mov     r8, r3
   0x00566c04 <+44>:    mov     r7, r2
   0x00566c08 <+48>:    mov     r6, r1
   0x00566c0c <+52>:    cmp     r1, r2
   0x00566c10 <+56>:    bne     0x566c20 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+72>
   0x00566c14 <+60>:    cmp     r7, #0
   0x00566c18 <+64>:    movne   r0, #1
   0x00566c1c <+68>:    strbne  r0, [r9, #52]   ; 0x34
   0x00566c20 <+72>:    ldr     r4, [r9]
   0x00566c24 <+76>:    ldr     r0, [r4, #140]  ; 0x8c
   0x00566c28 <+80>:    cmp     r0, #0
   0x00566c2c <+84>:    bne     0x566fe8 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+1040>
   0x00566c30 <+88>:    ldrb    r0, [r4, #144]  ; 0x90
   0x00566c34 <+92>:    mvn     r1, #0
   0x00566c38 <+96>:    str     r1, [r4, #140]  ; 0x8c
   0x00566c3c <+100>:   cmp     r0, #0
   0x00566c40 <+104>:   ldrbne  r0, [r4, #145]  ; 0x91
   0x00566c44 <+108>:   cmpne   r0, #0
   0x00566c48 <+112>:   bne     0x566dc8 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+496>
   0x00566c4c <+116>:   vmov    s0, r7
   0x00566c50 <+120>:   vldr    d8, [pc, #1008] ; 0x567048 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+1136>
=> 0x00566c54 <+124>:   vcvt.f64.u32    d16, s0
   0x00566c58 <+128>:   vmov    s0, r6
(gdb) disassemble
Dump of assembler code for function _ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568:
   0x00566bd8 <+0>:     push    {r4, r5, r6, r7, r8, r9, r10, r11, lr}
   0x00566bdc <+4>:     sub     sp, sp, #4
   0x00566be0 <+8>:     vpush   {d8}
   0x00566be4 <+12>:    sub     sp, sp, #72     ; 0x48
   0x00566be8 <+16>:    mov     r9, r0
   0x00566bec <+20>:    ldrb    r0, [r0, #52]   ; 0x34
   0x00566bf0 <+24>:    mov     r4, #0
   0x00566bf4 <+28>:    mov     r5, #0
   0x00566bf8 <+32>:    cmp     r0, #0
   0x00566bfc <+36>:    bne     0x566fd0 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+1016>
   0x00566c00 <+40>:    mov     r8, r3
   0x00566c04 <+44>:    mov     r7, r2
   0x00566c08 <+48>:    mov     r6, r1
   0x00566c0c <+52>:    cmp     r1, r2
   0x00566c10 <+56>:    bne     0x566c20 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+72>
   0x00566c14 <+60>:    cmp     r7, #0
   0x00566c18 <+64>:    movne   r0, #1
   0x00566c1c <+68>:    strbne  r0, [r9, #52]   ; 0x34
   0x00566c20 <+72>:    ldr     r4, [r9]
   0x00566c24 <+76>:    ldr     r0, [r4, #140]  ; 0x8c
   0x00566c28 <+80>:    cmp     r0, #0
   0x00566c2c <+84>:    bne     0x566fe8 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+1040>
   0x00566c30 <+88>:    ldrb    r0, [r4, #144]  ; 0x90
   0x00566c34 <+92>:    mvn     r1, #0
   0x00566c38 <+96>:    str     r1, [r4, #140]  ; 0x8c
   0x00566c3c <+100>:   cmp     r0, #0
   0x00566c40 <+104>:   ldrbne  r0, [r4, #145]  ; 0x91
   0x00566c44 <+108>:   cmpne   r0, #0
   0x00566c48 <+112>:   bne     0x566dc8 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+496>
   0x00566c4c <+116>:   vmov    s0, r7
   0x00566c50 <+120>:   vldr    d8, [pc, #1008] ; 0x567048 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+1136>
=> 0x00566c54 <+124>:   vcvt.f64.u32    d16, s0
   0x00566c58 <+128>:   vmov    s0, r6
--Type <RET> for more, q to quit, c to continue without paging--
   0x00566c5c <+132>:   vcvt.f64.u32    d17, s0
   0x00566c60 <+136>:   ldr     r2, [r4, #140]  ; 0x8c
   0x00566c64 <+140>:   add     r2, r2, #1
   0x00566c68 <+144>:   str     r2, [r4, #140]  ; 0x8c
   0x00566c6c <+148>:   stmib   sp, {r6, r7}
   0x00566c70 <+152>:   vdiv.f64        d16, d17, d16
   0x00566c74 <+156>:   vmov    r0, r1, d16
   0x00566c78 <+160>:   vldr    d17, [pc, #1008]        ; 0x567070 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+1176>
   0x00566c7c <+164>:   bic     r1, r1, #-2147483648    ; 0x80000000
   0x00566c80 <+168>:   vmov    d18, r0, r1
   0x00566c84 <+172>:   ldrb    r0, [r9, #12]
   0x00566c88 <+176>:   vcmpe.f64       d18, d17
   0x00566c8c <+180>:   vmrs    APSR_nzcv, fpscr
   0x00566c90 <+184>:   vmovmi.f64      d8, d16
   0x00566c94 <+188>:   cmp     r0, #1
   0x00566c98 <+192>:   bne     0x566cec <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+276>
   0x00566c9c <+196>:   mov     r1, #2
   0x00566ca0 <+200>:   add     r2, sp, #56     ; 0x38
   0x00566ca4 <+204>:   ldr     r0, [pc, #980]  ; 0x567080 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+1192>
   0x00566ca8 <+208>:   str     r1, [sp, #52]   ; 0x34
   0x00566cac <+212>:   str     r2, [sp, #48]   ; 0x30
   0x00566cb0 <+216>:   mov     r2, #0
   0x00566cb4 <+220>:   str     r2, [sp, #44]   ; 0x2c
   0x00566cb8 <+224>:   add     r0, pc, r0
   0x00566cbc <+228>:   str     r2, [sp, #40]   ; 0x28
   0x00566cc0 <+232>:   add     r2, sp, #8
   0x00566cc4 <+236>:   str     r1, [sp, #36]   ; 0x24
   0x00566cc8 <+240>:   ldr     r1, [pc, #948]  ; 0x567084 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+1196>
   0x00566ccc <+244>:   ldr     r1, [pc, r1]
   0x00566cd0 <+248>:   str     r1, [sp, #68]   ; 0x44
   0x00566cd4 <+252>:   str     r2, [sp, #64]   ; 0x40
   0x00566cd8 <+256>:   str     r1, [sp, #60]   ; 0x3c
   0x00566cdc <+260>:   add     r1, sp, #4
   0x00566ce0 <+264>:   str     r1, [sp, #56]   ; 0x38
--Type <RET> for more, q to quit, c to continue without paging--
   0x00566ce4 <+268>:   str     r0, [sp, #32]
   0x00566ce8 <+272>:   b       0x566d40 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+360>
   0x00566cec <+276>:   vldr    d16, [pc, #900] ; 0x567078 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+1184>
   0x00566cf0 <+280>:   mov     r2, #1
   0x00566cf4 <+284>:   ldr     r0, [pc, #936]  ; 0x5670a4 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+1228>
   0x00566cf8 <+288>:   add     r3, sp, #24
   0x00566cfc <+292>:   vmul.f64        d16, d8, d16
   0x00566d00 <+296>:   ldr     r1, [pc, #928]  ; 0x5670a8 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+1232>
   0x00566d04 <+300>:   str     r2, [sp, #52]   ; 0x34
   0x00566d08 <+304>:   add     r0, pc, r0
   0x00566d0c <+308>:   str     r3, [sp, #48]   ; 0x30
   0x00566d10 <+312>:   add     r1, pc, r1
   0x00566d14 <+316>:   str     r2, [sp, #44]   ; 0x2c
   0x00566d18 <+320>:   mov     r2, #2
   0x00566d1c <+324>:
        str     r2, [sp, #36]   ; 0x24
   0x00566d20 <+328>:   ldr     r2, [pc, #900]  ; 0x5670ac <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+1236>
   0x00566d24 <+332>:   ldr     r2, [pc, r2]
   0x00566d28 <+336>:   str     r2, [sp, #28]
   0x00566d2c <+340>:   add     r2, sp, #56     ; 0x38
   0x00566d30 <+344>:   str     r1, [sp, #40]   ; 0x28
   0x00566d34 <+348>:   str     r2, [sp, #24]
   0x00566d38 <+352>:   str     r0, [sp, #32]
   0x00566d3c <+356>:   vstr    d16, [sp, #56]  ; 0x38
   0x00566d40 <+360>:   add     r0, sp, #12
   0x00566d44 <+364>:   add     r1, sp, #32
   0x00566d48 <+368>:   bl      0xcaac20 <alloc::fmt::format>
   0x00566d4c <+372>:   ldr     r11, [sp, #20]
   0x00566d50 <+376>:   ldr     r4, [r9, #4]
   0x00566d54 <+380>:   ldr     r0, [r9, #8]
   0x00566d58 <+384>:   add     r1, r11, #17
   0x00566d5c <+388>:   cmp     r4, r0
   0x00566d60 <+392>:   movls   r0, r4
   0x00566d64 <+396>:   subs    r7, r0, r1
   0x00566d68 <+400>:   bcs     0x566d98 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+448>
--Type <RET> for more, q to quit, c to continue without paging--
   0x00566d6c <+404>:   ldr     r1, [sp, #16]
   0x00566d70 <+408>:   mov     r4, #0
   0x00566d74 <+412>:   str     r4, [sp, #64]   ; 0x40
   0x00566d78 <+416>:   cmp     r1, #0
   0x00566d7c <+420>:   str     r4, [sp, #60]   ; 0x3c
   0x00566d80 <+424>:   str     r4, [sp, #56]   ; 0x38
   0x00566d84 <+428>:   beq     0x566fcc <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+1012>
   0x00566d88 <+432>:   ldr     r0, [sp, #12]
   0x00566d8c <+436>:   mov     r2, #1
   0x00566d90 <+440>:   bl      0x49c8bc <__rust_dealloc>
   0x00566d94 <+444>:   b       0x566fcc <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+1012>
   0x00566d98 <+448>:   cmn     r4, #1
   0x00566d9c <+452>:   ble     0x567008 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+1072>
   0x00566da0 <+456>:   cmp     r4, #0
   0x00566da4 <+460>:   beq     0x566e00 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+552>
   0x00566da8 <+464>:   mov     r0, r4
   0x00566dac <+468>:   mov     r1, #1
   0x00566db0 <+472>:   bl      0x49c8b8 <__rust_alloc>
   0x00566db4 <+476>:   cmp     r0, #0
   0x00566db8 <+480>:   beq     0x567010 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+1080>
   0x00566dbc <+484>:   mov     r1, #0
   0x00566dc0 <+488>:   str     r1, [sp, #40]   ; 0x28
   0x00566dc4 <+492>:   b       0x566e28 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+592>
   0x00566dc8 <+496>:   mov     r1, #19
   0x00566dcc <+500>:   mov     r0, #0
   0x00566dd0 <+504>:   orr     r1, r1, #21504  ; 0x5400
   0x00566dd4 <+508>:   add     r2, sp, #32
   0x00566dd8 <+512>:   str     r0, [sp, #36]   ; 0x24
   0x00566ddc <+516>:   str     r0, [sp, #32]
   0x00566de0 <+520>:   mov     r0, #2
   0x00566de4 <+524>:   bl      0x43bc38 <ioctl@plt>
   0x00566de8 <+528>:   cmn     r0, #1
   0x00566dec <+532>:
        ble     0x566c4c <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+116>
   0x00566df0 <+536>:   ldrh    r0, [sp, #34]   ; 0x22
--Type <RET> for more, q to quit, c to continue without paging--
   0x00566df4 <+540>:   cmp     r0, #0
   0x00566df8 <+544>:   strne   r0, [r9, #4]
   0x00566dfc <+548>:   b       0x566c4c <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+116>
   0x00566e00 <+552>:   mov     r0, #0
   0x00566e04 <+556>:   mov     r1, #1
   0x00566e08 <+560>:   str     r0, [sp, #40]   ; 0x28
   0x00566e0c <+564>:   mov     r0, #1
   0x00566e10 <+568>:   str     r4, [sp, #36]   ; 0x24
   0x00566e14 <+572>:   mov     r4, #1
   0x00566e18 <+576>:   str     r4, [sp, #32]
   0x00566e1c <+580>:   bl      0x49c8b8 <__rust_alloc>
   0x00566e20 <+584>:   cmp     r0, #0
   0x00566e24 <+588>:   beq     0x567018 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+1088>
   0x00566e28 <+592>:   vmov    s0, r7
   0x00566e2c <+596>:   str     r4, [sp, #36]   ; 0x24
   0x00566e30 <+600>:   str     r0, [sp, #32]
   0x00566e34 <+604>:   mov     r1, #91 ; 0x5b
   0x00566e38 <+608>:   vcvt.f64.u32    d16, s0
   0x00566e3c <+612>:   vmul.f64        d16, d8, d16
   0x00566e40 <+616>:   vcvt.u32.f64    s0, d16
   0x00566e44 <+620>:   strb    r1, [r0]
   0x00566e48 <+624>:   mov     r0, #1
   0x00566e4c <+628>:   str     r0, [sp, #40]   ; 0x28
   0x00566e50 <+632>:   vmov    r10, s0
   0x00566e54 <+636>:   cmp     r10, #0
   0x00566e58 <+640>:   beq     0x566ec4 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+748>
   0x00566e5c <+644>:   ldr     r4, [pc, #548]  ; 0x567088 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+1200>
   0x00566e60 <+648>:
add     r5, sp, #32
   0x00566e64 <+652>:   mov     r6, r10
   0x00566e68 <+656>:   add     r4, pc, r4
   0x00566e6c <+660>:   subs    r6, r6, #1
   0x00566e70 <+664>:   beq     0x566e88 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+688>
   0x00566e74 <+668>:   mov     r0, r5
   0x00566e78 <+672>:   mov     r1, r4
--Type <RET> for more, q to quit, c to continue without paging--
   0x00566e7c <+676>:   mov     r2, #1
   0x00566e80 <+680>:   bl      0x67a490 <_ZN93_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$RF$T$GT$$GT$6extend17h135f0dd5c0b278d6E>
   0x00566e84 <+684>:   b       0x566e6c <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+660>
   0x00566e88 <+688>:   ldr     r0, [sp, #8]
   0x00566e8c <+692>:   ldr     r1, [sp, #4]
   0x00566e90 <+696>:   cmp     r1, r0
   0x00566e94 <+700>:   bne     0x566eb0 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+728>
   0x00566e98 <+704>:   ldr     r1, [pc, #496]  ; 0x567090 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+1208>
   0x00566e9c <+708>:   add     r1, pc, r1
   0x00566ea0 <+712>:   add     r0, sp, #32
   0x00566ea4 <+716>:   mov     r2, #1
   0x00566ea8 <+720>:   bl      0x67a490 <_ZN93_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$RF$T$GT$$GT$6extend17h135f0dd5c0b278d6E>
   0x00566eac <+724>:   b       0x566ec4 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+748>
   0x00566eb0 <+728>:   ldr     r1, [pc, #468]  ; 0x56708c <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+1204>
   0x00566eb4 <+732>:   add     r1, pc, r1
   0x00566eb8 <+736>:   add     r0, sp, #32
   0x00566ebc <+740>:   mov     r2, #1
   0x00566ec0 <+744>:   bl      0x67a490 <_ZN93_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$RF$T$GT$$GT$6extend17h135f0dd5c0b278d6E>
   0x00566ec4 <+748>:   ldr
r4, [pc, #456]  ; 0x567094 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+1212>
   0x00566ec8 <+752>:   sub     r6, r7, r10
   0x00566ecc <+756>:   add     r5, sp, #32
   0x00566ed0 <+760>:   mvn     r7, #0
   0x00566ed4 <+764>:   add     r4, pc, r4
   0x00566ed8 <+768>:   add     r7, r7, #1
   0x00566edc <+772>:   cmp     r7, r6
   0x00566ee0 <+776>:   bcs     0x566ef8 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+800>
   0x00566ee4 <+780>:   mov     r0, r5
   0x00566ee8 <+784>:   mov     r1, r4
   0x00566eec <+788>:   mov     r2, #1
   0x00566ef0 <+792>:   bl      0x67a490 <_ZN93_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$RF$T$GT$$GT$6extend17h135f0dd5c0b278d6E>
   0x00566ef4 <+796>:   b       0x566ed8 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+768>
   0x00566ef8 <+800>:   ldr     r1, [pc, #408]  ; 0x567098 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+1216>
   0x00566efc <+804>:   add     r1, pc, r1
   0x00566f00 <+808>:   add     r0, sp, #32
--Type <RET> for more, q to quit, c to continue without paging--
   0x00566f04 <+812>:   mov     r2, #1
   0x00566f08 <+816>:   bl      0x67a490 <_ZN93_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$RF$T$GT$$GT$6extend17h135f0dd5c0b278d6E>
   0x00566f0c <+820>:   ldr     r4, [sp, #12]
   0x00566f10 <+824>:   add     r0, sp, #32
   0x00566f14 <+828>:   mov     r1, r4
   0x00566f18 <+832>:   mov     r2, r11
   0x00566f1c <+836>:   bl      0x67a490 <_ZN93_$LT$alloc..vec..Vec$LT$T$GT$$u20$as$u20$core..iter..traits..collect..Extend$LT$$RF$T$GT$$GT$6extend17h135f0dd5c0b278d6E>
   0x00566f20 <+840>:   add     r3, sp, #32
   0x00566f24 <+844>:   ldr     r1, [sp, #16]
   0x00566f28 <+848>:   add     r12, sp, #56    ; 0x38
   0x00566f2c <+852>:   ldm     r3, {r0, r2, r3}
   0x00566f30 <+856>:   cmp     r1, #0
   0x00566f34 <+860>:   stm     r12, {r0, r2, r3}
   0x00566f38 <+864>:   beq     0x566f48 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+880>

   0x00566f3c <+868>:   mov     r0, r4
   0x00566f40 <+872>:   mov     r2, #1
   0x00566f44 <+876>:   bl      0x49c8bc <__rust_dealloc>
   0x00566f48 <+880>:   ldr     r0, [sp, #56]   ; 0x38
   0x00566f4c <+884>:   cmp     r0, #0
   0x00566f50 <+888>:   beq     0x566fc8 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+1008>
   0x00566f54 <+892>:   ldr     r6, [sp, #56]   ; 0x38
   0x00566f58 <+896>:   ldr     r0, [sp, #60]   ; 0x3c
   0x00566f5c <+900>:   ldr     r2, [sp, #64]   ; 0x40
   0x00566f60 <+904>:   str     r2, [sp, #40]   ; 0x28
   0x00566f64 <+908>:   str     r0, [sp, #36]   ; 0x24
   0x00566f68 <+912>:   str     r6, [sp, #32]
   0x00566f6c <+916>:   ldr     r0, [sp, #120]  ; 0x78
   0x00566f70 <+920>:   mov     r1, r6
   0x00566f74 <+924>:   str     r0, [sp]
   0x00566f78 <+928>:   mov     r0, r9
   0x00566f7c <+932>:   mov     r3, r8
   0x00566f80 <+936>:   bl      0x5670b0 <_ZN5cargo4util8progress5State5print17h6dc31809966badaeE.llvm.6020787217439157568>
   0x00566f84 <+940>:   mov     r4, r0
   0x00566f88 <+944>:   cmp     r0, #0
--Type <RET> for more, q to quit, c to continue without paging--
   0x00566f8c <+948>:   beq     0x566fb0 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+984>
   0x00566f90 <+952>:   mov     r5, r1
   0x00566f94 <+956>:   ldr     r1, [sp, #36]   ; 0x24
   0x00566f98 <+960>:   cmp     r1, #0
   0x00566f9c <+964>:   beq     0x566fd0 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+1016>
   0x00566fa0 <+968>:   mov     r0, r6
   0x00566fa4 <+972>:   mov     r2, #1
   0x00566fa8 <+976>:   bl      0x49c8bc <__rust_dealloc>

   0x00566fac <+980>:   b       0x566fd0 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+1016>
   0x00566fb0 <+984>:   ldr     r1, [sp, #36]   ; 0x24
   0x00566fb4 <+988>:   cmp     r1, #0
   0x00566fb8 <+992>:   beq     0x566fc8 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+1008>
   0x00566fbc <+996>:   mov     r0, r6
   0x00566fc0 <+1000>:  mov     r2, #1
   0x00566fc4 <+1004>:  bl      0x49c8bc <__rust_dealloc>
   0x00566fc8 <+1008>:  mov     r4, #0
   0x00566fcc <+1012>:  mov     r5, #0
   0x00566fd0 <+1016>:  mov     r0, r4
   0x00566fd4 <+1020>:  mov     r1, r5
   0x00566fd8 <+1024>:  add     sp, sp, #72     ; 0x48
   0x00566fdc <+1028>:  vpop    {d8}
   0x00566fe0 <+1032>:  add     sp, sp, #4
   0x00566fe4 <+1036>:  pop     {r4, r5, r6, r7, r8, r9, r10, r11, pc}
   0x00566fe8 <+1040>:  ldr     r0, [pc, #172]  ; 0x56709c <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+1220>
   0x00566fec <+1044>:  add     r2, sp, #32
   0x00566ff0 <+1048>:  ldr     r3, [pc, #168]  ; 0x5670a0 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+1224>
   0x00566ff4 <+1052>:  mov     r1, #16
   0x00566ff8 <+1056>:  add     r0, pc, r0
   0x00566ffc <+1060>:  add     r3, pc, r3



   0x00567000 <+1064>:  bl      0xcb0c9c <core::result::unwrap_failed>
   0x00567004 <+1068>:  udf     #65006  ; 0xfdee
   0x00567008 <+1072>:  bl      0x750194 <_ZN5alloc7raw_vec19RawVec$LT$T$C$A$GT$11allocate_in28_$u7b$$u7b$closure$u7d$$u7d$17hfff8a4b022386362E.llvm.10649790427578084810>
   0x0056700c <+1076>:  udf     #65006  ; 0xfdee
   0x00567010 <+1080>:  mov     r0, r4
--Type <RET> for more, q to quit, c to continue without paging--
   0x00567014 <+1084>:  b       0x56701c <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+1092>
   0x00567018 <+1088>:
        mov     r0, #1
   0x0056701c <+1092>:  mov     r1, #1
   0x00567020 <+1096>:  bl      0xcaaa24 <alloc::alloc::handle_alloc_error>
   0x00567024 <+1100>:  udf     #65006  ; 0xfdee
   0x00567028 <+1104>:  mov     r4, r0
   0x0056702c <+1108>:  b       0x56705c <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+1156>
   0x00567030 <+1112>:  mov     r4, r0
   0x00567034 <+1116>:  add     r0, sp, #32
   0x00567038 <+1120>:  b       0x567060 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+1160>
   0x0056703c <+1124>:  b       0x567050 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+1144>
   0x00567040 <+1128>:  b       0x567050 <_ZN5cargo4util8progress5State4tick17hfa4ef476c56b997eE.llvm.6020787217439157568+1144>
   0x00567044 <+1132>:  nop                     ; (mov r0, r0)
   0x00567048 <+1136>:  andeq   r0, r0, r0
   0x0056704c <+1140>:  andeq   r0, r0, r0
   0x00567050 <+1144>:  mov     r4, r0
   0x00567054 <+1148>:  add     r0, sp, #32
   0x00567058 <+1152>:  bl      0x541f50 <_ZN4core3ptr18real_drop_in_place17hfc27c23e20d4480aE.llvm.6020787217439157568>
   0x0056705c <+1156>:  add     r0, sp, #12
   0x00567060 <+1160>:  bl      0x541f50 <_ZN4core3ptr18real_drop_in_place17hfc27c23e20d4480aE.llvm.6020787217439157568>
   0x00567064 <+1164>:  mov     r0, r4
   0x00567068 <+1168>:  bl      0x43b65c <_Unwind_Resume@plt>
   0x0056706c <+1172>:  udf     #65006  ; 0xfdee
   0x00567070 <+1176>:  andeq   r0, r0, r0
   0x00567074 <+1180>:  svcvc   0x00f00000      ; IMB
   0x00567078 <+1184>:  andeq   r0, r0, r0
   0x0056707c <+1188>:  subsmi  r0, r9, r0
   0x00567080 <+1192>:  addeq   sp, lr, r12, lsr #7
   0x00567084 <+1196>:  addseq  pc, r0, r8, lsl #6
   0x00567088 <+1200>:
        rsbseq  r5, r6, r4, lsl r12
   0x0056708c <+1204>:  rsbseq  r5, r6, r9, asr #23
   0x00567090 <+1208>:  rsbseq  r5, r6, r0, ror #23
   0x00567094 <+1212>:  rsbseq  r5, r6, sp, lsl #1
   0x00567098 <+1216>:  rsbseq  r5, r6, r2, lsl #23
--Type <RET> for more, q to quit, c to continue without paging--
   0x0056709c <+1220>:  rsbseq  lr, r5, r6, lsr #31
   0x005670a0 <+1224>:  addeq   pc, lr, r0, lsl #6
   0x005670a4 <+1228>:  addeq   sp, lr, r12, ror #6
   0x005670a8 <+1232>:  rsbseq  r5, r6, r8, asr #26
   0x005670ac <+1236>:                  ; <UNDEFINED> instruction: 0x0090eebc
End of assembler dump.

