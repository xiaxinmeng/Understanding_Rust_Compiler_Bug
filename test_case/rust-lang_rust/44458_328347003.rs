asm
00000000 <_ZN4core3ptr13drop_in_place17h032f94189ca657d1E>:
   0:   b570            push    {r4, r5, r6, lr}
   2:   4604            mov     r4, r0
   4:   6860            ldr     r0, [r4, #4]
   6:   b1a8            cbz     r0, 34 <_ZN4core3ptr13drop_in_place17h032f94189ca657d1E+0x34>
   8:   7a20            ldrb    r0, [r4, #8]
   a:   2802            cmp     r0, #2
   c:   bf38            it      cc
   e:   bd70            popcc   {r4, r5, r6, pc}
  10:   68e6            ldr     r6, [r4, #12]
  12:   e9d6 0101       ldrd    r0, r1, [r6, #4]
  16:   6809            ldr     r1, [r1, #0]
  18:   4788            blx     r1
  1a:   68b0            ldr     r0, [r6, #8]
  1c:   6841            ldr     r1, [r0, #4]
  1e:   b119            cbz     r1, 28 <_ZN4core3ptr13drop_in_place17h032f94189ca657d1E+0x28>
  20:   6882            ldr     r2, [r0, #8]
  22:   6870            ldr     r0, [r6, #4]
  24:   f7ff fffe       bl      0 <__rust_deallocate>
  28:   68e0            ldr     r0, [r4, #12]
  2a:   210c            movs    r1, #12
  2c:   2204            movs    r2, #4
  2e:   e8bd 4070       ldmia.w sp!, {r4, r5, r6, lr}
  32:   e7fe            b.n     0 <__rust_deallocate>
  34:   bd70            pop     {r4, r5, r6, pc}
  36:   4605            mov     r5, r0
  38:   68b0            ldr     r0, [r6, #8]
  3a:   6841            ldr     r1, [r0, #4]
  3c:   b119            cbz     r1, 46 <_ZN4core3ptr13drop_in_place17h032f94189ca657d1E+0x46>
  3e:   6882            ldr     r2, [r0, #8]
  40:   6870            ldr     r0, [r6, #4]
  42:   f7ff fffe       bl      0 <__rust_deallocate>
  46:   68e0            ldr     r0, [r4, #12]
  48:   210c            movs    r1, #12
  4a:   2204            movs    r2, #4
  4c:   f7ff fffe       bl      0 <__rust_deallocate>
  50:   4628            mov     r0, r5
  52:   f7ff fffe       bl      0 <_Unwind_Resume>
