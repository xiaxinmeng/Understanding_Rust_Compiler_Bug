 asm
08000134 <_start>:
 8000134:       b580            push    {r7, lr}
 8000136:       466f            mov     r7, sp
 8000138:       b088            sub     sp, #32
 800013a:       e7ff            b.n     800013c <_start+0x8>
 800013c:       2018            movs    r0, #24
 800013e:       9001            str     r0, [sp, #4]
 8000140:       202a            movs    r0, #42 ; 0x2a
 8000142:       9004            str     r0, [sp, #16]
 8000144:       9007            str     r0, [sp, #28]
 8000146:       a901            add     r1, sp, #4
 8000148:       9000            str     r0, [sp, #0]
 800014a:       4608            mov     r0, r1
 800014c:       9900            ldr     r1, [sp, #0]
 800014e:       f000 f803       bl      8000158 <core::ptr::write::h8f7040d3eb269e5e>
 8000152:       e7ff            b.n     8000154 <_start+0x20>
 8000154:       e7ff            b.n     8000156 <_start+0x22>
 8000156:       e7fe            b.n     8000156 <_start+0x22>

08000158 <core::ptr::write::h8f7040d3eb269e5e>:
 8000158:       b087            sub     sp, #28
 800015a:       460a            mov     r2, r1
 800015c:       4603            mov     r3, r0
 800015e:       9002            str     r0, [sp, #8]
 8000160:       9104            str     r1, [sp, #16]
 8000162:       9103            str     r1, [sp, #12]
 8000164:       9201            str     r2, [sp, #4]
 8000166:       9300            str     r3, [sp, #0]
 8000168:       e7ff            b.n     800016a <core::ptr::write::h8f7040d3eb269e5e+0x12>
 800016a:       9802            ldr     r0, [sp, #8]
 800016c:       9005            str     r0, [sp, #20]
 800016e:       9803            ldr     r0, [sp, #12]
 8000170:       9006            str     r0, [sp, #24]
 8000172:       9905            ldr     r1, [sp, #20]
 8000174:       6008            str     r0, [r1, #0]
 8000176:       e7ff            b.n     8000178 <core::ptr::write::h8f7040d3eb269e5e+0x20>
 8000178:       b007            add     sp, #28
 800017a:       4770            bx      lr
