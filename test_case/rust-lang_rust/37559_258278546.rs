 asm
             __powisf2:
0002af00         vmov.f32   s14, #0x1                                           
0002af04         mov        r3, r1
0002af06         vmov       s15, r0
0002af0a         b          0x2af10

0002af0c         vmul.f32   s15, s15, s15                                       

0002af10         add.w      r2, r3, r3, lsr #31                                 
0002af14         lsls       r3, r3, #0x1f
0002af16         it         mi
0002af18         vmulmi.f32 s14, s14, s15
0002af1c         asrs       r3, r2, #0x1
0002af1e         bne        0x2af0c

0002af20         cmp        r1, #0x0
0002af22         itt        lt
0002af24         vmovlt.f32 s15, #0x1
0002af28         vdivlt.f32 s14, s15, s14
0002af2c         vmov       r0, s14
0002af30         bx         lr

