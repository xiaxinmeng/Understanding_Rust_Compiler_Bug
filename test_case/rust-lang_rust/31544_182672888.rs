 asm
08000000 <_ZN10EXCEPTIONS20h77c550f11b0b768cAbaE>:
 8000000:       20002000        andcs   r2, r0, r0
 8000004:       08000009        stmdaeq r0, {r0, r3}

08000008 <__reset>:
 8000008:       b580            push    {r7, lr}
 800000a:       f240 0000       movw    r0, #0
 800000e:       f240 0100       movw    r1, #0
 8000012:       f2c2 0000       movt    r0, #8192       ; 0x2000
 8000016:       f2c2 0100       movt    r1, #8192       ; 0x2000
 800001a:       1a09            subs    r1, r1, r0
 800001c:       f021 0203       bic.w   r2, r1, #3
 8000020:       f240 0154       movw    r1, #84 ; 0x54
 8000024:       f6c0 0100       movt    r1, #2048       ; 0x800
 8000028:       f000 f806       bl      8000038 <__aeabi_memcpy4>
 800002c:       f240 0000       movw    r0, #0
 8000030:       f6c0 0000       movt    r0, #2048       ; 0x800
 8000034:       6800            ldr     r0, [r0, #0]
 8000036:       e7fe            b.n     8000036 <__reset+0x2e>

08000038 <__aeabi_memcpy4>:
 8000038:       f000 b800       b.w     800003c <memcpy>

0800003c <memcpy>:
 800003c:       2a00            cmp     r2, #0
 800003e:       bf08            it      eq
 8000040:       4770            bxeq    lr
 8000042:       4684            mov     ip, r0
 8000044:       f811 3b01       ldrb.w  r3, [r1], #1
 8000048:       3a01            subs    r2, #1
 800004a:       f80c 3b01       strb.w  r3, [ip], #1
 800004e:       d1f9            bne.n   8000044 <memcpy+0x8>
 8000050:       4770            bx      lr
