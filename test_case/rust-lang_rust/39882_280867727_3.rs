 asm
08000134 <_start>:
 8000134:       b580            push    {r7, lr}
 8000136:       466f            mov     r7, sp
 8000138:       b084            sub     sp, #16
 800013a:       e7ff            b.n     800013c <_start+0x8>
 800013c:       2018            movs    r0, #24
 800013e:       9001            str     r0, [sp, #4]
 8000140:       a801            add     r0, sp, #4
 8000142:       212a            movs    r1, #42 ; 0x2a
 8000144:       f000 f803       bl      800014e <core::ptr::write::hb7006988e1de10a2>
 8000148:       e7ff            b.n     800014a <_start+0x16>
 800014a:       e7ff            b.n     800014c <_start+0x18>
 800014c:       e7fe            b.n     800014c <_start+0x18>

0800014e <core::ptr::write::hb7006988e1de10a2>:
 800014e:       b086            sub     sp, #24
 8000150:       460a            mov     r2, r1
 8000152:       4603            mov     r3, r0
 8000154:       9002            str     r0, [sp, #8]
 8000156:       9103            str     r1, [sp, #12]
 8000158:       9201            str     r2, [sp, #4]
 800015a:       9300            str     r3, [sp, #0]
 800015c:       e7ff            b.n     800015e <core::ptr::write::hb7006988e1de10a2+0x10>
 800015e:       9802            ldr     r0, [sp, #8]
 8000160:       9004            str     r0, [sp, #16]
 8000162:       9803            ldr     r0, [sp, #12]
 8000164:       9005            str     r0, [sp, #20]
 8000166:       9904            ldr     r1, [sp, #16]
 8000168:       6008            str     r0, [r1, #0]
 800016a:       e7ff            b.n     800016c <core::ptr::write::hb7006988e1de10a2+0x1e>
 800016c:       b006            add     sp, #24
 800016e:       4770            bx      lr
