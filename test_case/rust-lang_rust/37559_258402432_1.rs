 asm
             __powisf2:
00000000         vmov.f32   s15, s0
00000004         eor.w      r3, r0, r0, asr #31
00000008         sub.w      r3, r3, r0, asr #31
0000000c         tst.w      r3, #0x1
00000010         vmov.f32   s0, #0x1
00000014         it         ne
00000016         vmovne.f32 s0, s15
0000001a         b          0x28

0000001c         vmul.f32   s15, s15, s15                                       ; XREF=__powisf2+42
00000020         lsls       r2, r3, #0x1f
00000022         it         mi
00000024         vmulmi.f32 s0, s0, s15

00000028         lsrs       r3, r3, #0x1                                        ; XREF=__powisf2+26
0000002a         bne        0x1c

0000002c         cmp        r0, #0x0
0000002e         itt        lt
00000030         vmovlt.f32 s15, #0x1
00000034         vdivlt.f32 s0, s15, s0
00000038         bx         lr
                        ; endp
0000003a         nop    
