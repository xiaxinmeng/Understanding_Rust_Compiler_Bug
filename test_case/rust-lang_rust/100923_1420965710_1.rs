asm
80007120 <luma_core::gx::bp::Bp::set_vertical_filter>:
80007120:	7c 08 02 a6 	mflr    r0
80007124:	90 01 00 04 	stw     r0,4(r1)
80007128:	94 21 ff 70 	stwu    r1,-144(r1)
8000712c:	81 03 00 00 	lwz     r8,0(r3)
80007130:	38 a0 00 07 	li      r5,7
80007134:	80 63 00 03 	lwz     r3,3(r3)
80007138:	38 81 00 10 	addi    r4,r1,16
8000713c:	3c c0 80 00 	lis     r6,-32768
80007140:	3c e0 80 01 	lis     r7,-32767
80007144:	3d 20 80 01 	lis     r9,-32767
80007148:	92 81 00 60 	stw     r20,96(r1)
# ↓ r4 points to what will eventually be five bytes before the start of the array on the stack.
8000714c:	50 a4 07 7e 	rlwimi  r4,r5,0,29,31
80007150:	92 a1 00 64 	stw     r21,100(r1)
80007154:	3a a7 83 28 	addi    r21,r7,-31960
80007158:	92 c1 00 68 	stw     r22,104(r1)
8000715c:	3a c6 63 dc 	addi    r22,r6,25564
80007160:	92 e1 00 6c 	stw     r23,108(r1)
80007164:	3a e1 00 28 	addi    r23,r1,40
80007168:	93 01 00 70 	stw     r24,112(r1)
8000716c:	3b 00 00 01 	li      r24,1
80007170:	93 21 00 74 	stw     r25,116(r1)
80007174:	3b 21 00 27 	addi    r25,r1,39
80007178:	93 41 00 78 	stw     r26,120(r1)
8000717c:	3b 40 00 00 	li      r26,0
80007180:	93 61 00 7c 	stw     r27,124(r1)
80007184:	3b 60 00 08 	li      r27,8
80007188:	93 81 00 80 	stw     r28,128(r1)
8000718c:	3b 89 82 38 	addi    r28,r9,-32200
80007190:	93 a1 00 84 	stw     r29,132(r1)
80007194:	3b a1 00 40 	addi    r29,r1,64
80007198:	93 c1 00 88 	stw     r30,136(r1)
8000719c:	3b c1 00 30 	addi    r30,r1,48
800071a0:	3a 80 00 00 	li      r20,0
800071a4:	92 61 00 5c 	stw     r19,92(r1)
# ↓ The next two instructions store the array on the stack, five bytes after r4.
800071a8:	90 a1 00 14 	stw     r5,20(r1)
800071ac:	91 01 00 18 	stw     r8,24(r1)
800071b0:	90 61 00 1b 	stw     r3,27(r1)
800071b4:	28 1b 00 0f 	cmplwi  r27,15
800071b8:	41 82 00 90 	beq     80007248 <luma_core::gx::bp::Bp::set_vertical_filter+0x128>
# ↓ This is where each byte gets loaded, one byte later than r4.
800071bc:	88 64 00 01 	lbz     r3,1(r4)
800071c0:	3a 64 00 01 	addi    r19,r4,1
800071c4:	7f 84 e3 78 	mr      r4,r28
800071c8:	7f a5 eb 78 	mr      r5,r29
800071cc:	98 61 00 27 	stb     r3,39(r1)
800071d0:	7f c3 f3 78 	mr      r3,r30
800071d4:	93 21 00 28 	stw     r25,40(r1)
800071d8:	92 c1 00 2c 	stw     r22,44(r1)
800071dc:	93 41 00 30 	stw     r26,48(r1)
800071e0:	93 01 00 34 	stw     r24,52(r1)
800071e4:	93 41 00 38 	stw     r26,56(r1)
800071e8:	93 41 00 40 	stw     r26,64(r1)
800071ec:	92 a1 00 48 	stw     r21,72(r1)
800071f0:	93 01 00 4c 	stw     r24,76(r1)
800071f4:	92 e1 00 50 	stw     r23,80(r1)
800071f8:	93 01 00 54 	stw     r24,84(r1)
800071fc:	4b ff ed fd 	bl      80005ff8 <core::fmt::write>
80007200:	28 03 00 00 	cmplwi  r3,0
80007204:	40 c2 00 84 	bne-    80007288 <luma_core::gx::bp::Bp::set_vertical_filter+0x168>
80007208:	80 61 00 38 	lwz     r3,56(r1)
8000720c:	7f a5 eb 78 	mr      r5,r29
80007210:	80 81 00 34 	lwz     r4,52(r1)
80007214:	90 61 00 40 	stw     r3,64(r1)
80007218:	48 00 00 a1 	bl      800072b8 <__write_console>
8000721c:	80 81 00 30 	lwz     r4,48(r1)
80007220:	28 04 00 00 	cmplwi  r4,0
80007224:	41 82 00 14 	beq     80007238 <luma_core::gx::bp::Bp::set_vertical_filter+0x118>
80007228:	7c 83 20 f8 	not     r3,r4
8000722c:	54 65 0f fe 	srwi    r5,r3,31
80007230:	80 61 00 34 	lwz     r3,52(r1)
80007234:	48 00 04 bd 	bl      800076f0 <__rust_dealloc>
80007238:	3b 7b 00 01 	addi    r27,r27,1
8000723c:	3a 94 00 01 	addi    r20,r20,1
80007240:	7e 64 9b 78 	mr      r4,r19
80007244:	4b ff ff 70 	b       800071b4 <luma_core::gx::bp::Bp::set_vertical_filter+0x94>
80007248:	83 c1 00 88 	lwz     r30,136(r1)
8000724c:	83 a1 00 84 	lwz     r29,132(r1)
80007250:	83 81 00 80 	lwz     r28,128(r1)
80007254:	83 61 00 7c 	lwz     r27,124(r1)
80007258:	83 41 00 78 	lwz     r26,120(r1)
8000725c:	83 21 00 74 	lwz     r25,116(r1)
80007260:	83 01 00 70 	lwz     r24,112(r1)
80007264:	82 e1 00 6c 	lwz     r23,108(r1)
80007268:	82 c1 00 68 	lwz     r22,104(r1)
8000726c:	82 a1 00 64 	lwz     r21,100(r1)
80007270:	82 81 00 60 	lwz     r20,96(r1)
80007274:	82 61 00 5c 	lwz     r19,92(r1)
80007278:	80 01 00 94 	lwz     r0,148(r1)
8000727c:	38 21 00 90 	addi    r1,r1,144
80007280:	7c 08 03 a6 	mtlr    r0
80007284:	4e 80 00 20 	blr
80007288:	38 7b ff f9 	addi    r3,r27,-7
8000728c:	90 61 00 10 	stw     r3,16(r1)
80007290:	3c 60 80 00 	lis     r3,-32768
80007294:	3c 80 80 01 	lis     r4,-32767
80007298:	3c e0 80 01 	lis     r7,-32767
8000729c:	38 a1 00 40 	addi    r5,r1,64
800072a0:	38 63 7f c2 	addi    r3,r3,32706
800072a4:	38 c4 82 50 	addi    r6,r4,-32176
800072a8:	38 e7 83 08 	addi    r7,r7,-31992
800072ac:	38 80 00 37 	li      r4,55
800072b0:	4b ff f0 b1 	bl      80006360 <core::result::unwrap_failed>
800072b4:	7f e0 00 08 	trap
