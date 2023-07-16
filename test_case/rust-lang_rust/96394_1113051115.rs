
Starting program: /volume1/incomming/mqtt2file
warning: Unable to find libthread_db matching inferior's thread library, thread debugging will not be available.

Program received signal SIGILL, Illegal instruction.
0x206fa57c in core::sync::atomic::atomic_compare_exchange (dst=0x208b9484 <std::thread::ThreadId::new::GUARD>, old=0,
    new=1, success=core::sync::atomic::Ordering::Acquire, failure=core::sync::atomic::Ordering::Relaxed)
    at /home/mortenlj/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs:2643
2643	/home/mortenlj/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/sync/atomic.rs: No such file or directory.
(gdb) disassemble
Dump of assembler code for function core::sync::atomic::atomic_compare_exchange:
   0x206fa200 <+0>:	mflr    r0
   0x206fa204 <+4>:	stw     r0,4(r1)
   0x206fa208 <+8>:	stwu    r1,-208(r1)
   0x206fa20c <+12>:	stw     r30,200(r1)
   0x206fa210 <+16>:	stw     r5,64(r1)
   0x206fa214 <+20>:	stw     r4,68(r1)
   0x206fa218 <+24>:	stw     r3,72(r1)
   0x206fa21c <+28>:	bl      0x206fa220 <core::sync::atomic::atomic_compare_exchange+32>
   0x206fa220 <+32>:	mflr    r30
   0x206fa224 <+36>:	lwz     r8,-36(r30)
   0x206fa228 <+40>:	add     r30,r8,r30
   0x206fa22c <+44>:	stw     r3,176(r1)
   0x206fa230 <+48>:	stw     r4,180(r1)
   0x206fa234 <+52>:	stw     r5,184(r1)
   0x206fa238 <+56>:	stb     r6,190(r1)
   0x206fa23c <+60>:	stb     r7,191(r1)
   0x206fa240 <+64>:	stb     r6,96(r1)
   0x206fa244 <+68>:	stb     r7,97(r1)
   0x206fa248 <+72>:	lbz     r3,96(r1)
   0x206fa24c <+76>:	stw     r3,76(r1)
   0x206fa250 <+80>:	lwz     r3,76(r1)
   0x206fa254 <+84>:	rlwinm  r3,r3,2,0,29
   0x206fa258 <+88>:	lwz     r4,-32596(r30)
   0x206fa25c <+92>:	lwzx    r3,r3,r4
   0x206fa260 <+96>:	add     r3,r3,r4
   0x206fa264 <+100>:	mtctr   r3
   0x206fa268 <+104>:	bctr
   0x206fa26c <+108>:	lbz     r3,97(r1)
   0x206fa270 <+112>:	cmplwi  r3,0
   0x206fa274 <+116>:	beq-    0x206fa644 <core::sync::atomic::atomic_compare_exchange+1092>
   0x206fa278 <+120>:	b       0x206fa308 <core::sync::atomic::atomic_compare_exchange+264>
   0x206fa27c <+124>:	lbz     r3,97(r1)
   0x206fa280 <+128>:	cmplwi  r3,0
   0x206fa284 <+132>:	beq-    0x206fa5e8 <core::sync::atomic::atomic_compare_exchange+1000>
   0x206fa288 <+136>:	b       0x206fa308 <core::sync::atomic::atomic_compare_exchange+264>
   0x206fa28c <+140>:	lbz     r3,97(r1)
   0x206fa290 <+144>:	stw     r3,60(r1)
   0x206fa294 <+148>:	cmplwi  r3,0
   0x206fa298 <+152>:	beq-    0x206fa530 <core::sync::atomic::atomic_compare_exchange+816>
   0x206fa29c <+156>:	b       0x206fa2a0 <core::sync::atomic::atomic_compare_exchange+160>
   0x206fa2a0 <+160>:	lwz     r3,60(r1)
   0x206fa2a4 <+164>:	cmplwi  r3,2
   0x206fa2a8 <+168>:	beq-    0x206fa58c <core::sync::atomic::atomic_compare_exchange+908>
   0x206fa2ac <+172>:	b       0x206fa308 <core::sync::atomic::atomic_compare_exchange+264>
   0x206fa2b0 <+176>:	lbz     r3,97(r1)
   0x206fa2b4 <+180>:	stw     r3,56(r1)
   0x206fa2b8 <+184>:	cmplwi  r3,0
   0x206fa2bc <+188>:	beq-    0x206fa470 <core::sync::atomic::atomic_compare_exchange+624>
   0x206fa2c0 <+192>:	b       0x206fa2c4 <core::sync::atomic::atomic_compare_exchange+196>
   0x206fa2c4 <+196>:	lwz     r3,56(r1)
---Type <return> to continue, or q <return> to quit---
   0x206fa2c8 <+200>:	cmplwi  r3,2
   0x206fa2cc <+204>:	beq-    0x206fa4d0 <core::sync::atomic::atomic_compare_exchange+720>
   0x206fa2d0 <+208>:	b       0x206fa308 <core::sync::atomic::atomic_compare_exchange+264>
   0x206fa2d4 <+212>:	lbz     r3,97(r1)
   0x206fa2d8 <+216>:	stw     r3,52(r1)
   0x206fa2dc <+220>:	cmplwi  r3,0
   0x206fa2e0 <+224>:	beq-    0x206fa32c <core::sync::atomic::atomic_compare_exchange+300>
   0x206fa2e4 <+228>:	b       0x206fa2e8 <core::sync::atomic::atomic_compare_exchange+232>
   0x206fa2e8 <+232>:	lwz     r3,52(r1)
   0x206fa2ec <+236>:	cmplwi  r3,2
   0x206fa2f0 <+240>:	beq-    0x206fa38c <core::sync::atomic::atomic_compare_exchange+396>
   0x206fa2f4 <+244>:	b       0x206fa2f8 <core::sync::atomic::atomic_compare_exchange+248>
   0x206fa2f8 <+248>:	lwz     r3,52(r1)
   0x206fa2fc <+252>:	cmplwi  r3,4
   0x206fa300 <+256>:	beq-    0x206fa3ec <core::sync::atomic::atomic_compare_exchange+492>
   0x206fa304 <+260>:	b       0x206fa308 <core::sync::atomic::atomic_compare_exchange+264>
   0x206fa308 <+264>:	lbz     r3,97(r1)
   0x206fa30c <+268>:	stw     r3,48(r1)
   0x206fa310 <+272>:	cmplwi  r3,1
   0x206fa314 <+276>:	beq-    0x206fa6b8 <core::sync::atomic::atomic_compare_exchange+1208>
   0x206fa318 <+280>:	b       0x206fa31c <core::sync::atomic::atomic_compare_exchange+284>
   0x206fa31c <+284>:	lwz     r3,48(r1)
   0x206fa320 <+288>:	cmplwi  r3,3
   0x206fa324 <+292>:	beq-    0x206fa6d4 <core::sync::atomic::atomic_compare_exchange+1236>
   0x206fa328 <+296>:	b       0x206fa69c <core::sync::atomic::atomic_compare_exchange+1180>
   0x206fa32c <+300>:	msync
   0x206fa330 <+304>:	lwz     r3,68(r1)
   0x206fa334 <+308>:	lwz     r4,72(r1)
   0x206fa338 <+312>:	lwarx   r4,0,r4
   0x206fa33c <+316>:	stw     r4,44(r1)
   0x206fa340 <+320>:	cmpw    r3,r4
   0x206fa344 <+324>:	bne-    0x206fa35c <core::sync::atomic::atomic_compare_exchange+348>
   0x206fa348 <+328>:	lwz     r3,64(r1)
   0x206fa34c <+332>:	lwz     r4,72(r1)
   0x206fa350 <+336>:	stwcx.  r3,0,r4
   0x206fa354 <+340>:	bne+    0x206fa330 <core::sync::atomic::atomic_compare_exchange+304>
   0x206fa358 <+344>:	b       0x206fa368 <core::sync::atomic::atomic_compare_exchange+360>
   0x206fa35c <+348>:	lwz     r3,44(r1)
   0x206fa360 <+352>:	lwz     r4,72(r1)
   0x206fa364 <+356>:	stwcx.  r3,0,r4
   0x206fa368 <+360>:	lwz     r4,44(r1)
   0x206fa36c <+364>:	lwz     r3,68(r1)
   0x206fa370 <+368>:	xor     r3,r4,r3
   0x206fa374 <+372>:	cntlzw  r3,r3
   0x206fa378 <+376>:	rlwinm  r3,r3,27,5,31
   0x206fa37c <+380>:	.long 0x7c2004ac
   0x206fa380 <+384>:	stw     r4,88(r1)
   0x206fa384 <+388>:	stb     r3,92(r1)
   0x206fa388 <+392>:	b       0x206fa44c <core::sync::atomic::atomic_compare_exchange+588>
   0x206fa38c <+396>:	msync
   0x206fa390 <+400>:	lwz     r3,68(r1)
---Type <return> to continue, or q <return> to quit---
   0x206fa394 <+404>:	lwz     r4,72(r1)
   0x206fa398 <+408>:	lwarx   r4,0,r4
   0x206fa39c <+412>:	stw     r4,40(r1)
   0x206fa3a0 <+416>:	cmpw    r3,r4
   0x206fa3a4 <+420>:	bne-    0x206fa3bc <core::sync::atomic::atomic_compare_exchange+444>
   0x206fa3a8 <+424>:	lwz     r3,64(r1)
   0x206fa3ac <+428>:	lwz     r4,72(r1)
   0x206fa3b0 <+432>:	stwcx.  r3,0,r4
   0x206fa3b4 <+436>:	bne+    0x206fa390 <core::sync::atomic::atomic_compare_exchange+400>
   0x206fa3b8 <+440>:	b       0x206fa3c8 <core::sync::atomic::atomic_compare_exchange+456>
   0x206fa3bc <+444>:	lwz     r3,40(r1)
   0x206fa3c0 <+448>:	lwz     r4,72(r1)
   0x206fa3c4 <+452>:	stwcx.  r3,0,r4
   0x206fa3c8 <+456>:	lwz     r4,40(r1)
   0x206fa3cc <+460>:	lwz     r3,68(r1)
   0x206fa3d0 <+464>:	xor     r3,r4,r3
   0x206fa3d4 <+468>:	cntlzw  r3,r3
   0x206fa3d8 <+472>:	rlwinm  r3,r3,27,5,31
   0x206fa3dc <+476>:	.long 0x7c2004ac
   0x206fa3e0 <+480>:	stw     r4,88(r1)
   0x206fa3e4 <+484>:	stb     r3,92(r1)
   0x206fa3e8 <+488>:	b       0x206fa44c <core::sync::atomic::atomic_compare_exchange+588>
   0x206fa3ec <+492>:	msync
   0x206fa3f0 <+496>:	lwz     r3,68(r1)
   0x206fa3f4 <+500>:	lwz     r4,72(r1)
   0x206fa3f8 <+504>:	lwarx   r4,0,r4
   0x206fa3fc <+508>:	stw     r4,36(r1)
   0x206fa400 <+512>:	cmpw    r3,r4
   0x206fa404 <+516>:	bne-    0x206fa41c <core::sync::atomic::atomic_compare_exchange+540>
   0x206fa408 <+520>:	lwz     r3,64(r1)
   0x206fa40c <+524>:	lwz     r4,72(r1)
   0x206fa410 <+528>:	stwcx.  r3,0,r4
   0x206fa414 <+532>:	bne+    0x206fa3f0 <core::sync::atomic::atomic_compare_exchange+496>
   0x206fa418 <+536>:	b       0x206fa428 <core::sync::atomic::atomic_compare_exchange+552>
   0x206fa41c <+540>:	lwz     r3,36(r1)
   0x206fa420 <+544>:	lwz     r4,72(r1)
   0x206fa424 <+548>:	stwcx.  r3,0,r4
   0x206fa428 <+552>:	lwz     r4,36(r1)
   0x206fa42c <+556>:	lwz     r3,68(r1)
   0x206fa430 <+560>:	xor     r3,r4,r3
   0x206fa434 <+564>:	cntlzw  r3,r3
   0x206fa438 <+568>:	rlwinm  r3,r3,27,5,31
   0x206fa43c <+572>:	.long 0x7c2004ac
   0x206fa440 <+576>:	stw     r4,88(r1)
   0x206fa444 <+580>:	stb     r3,92(r1)
   0x206fa448 <+584>:	b       0x206fa44c <core::sync::atomic::atomic_compare_exchange+588>
   0x206fa44c <+588>:	lwz     r3,88(r1)
   0x206fa450 <+592>:	stw     r3,32(r1)
   0x206fa454 <+596>:	stw     r3,192(r1)
   0x206fa458 <+600>:	lbz     r4,92(r1)
   0x206fa45c <+604>:	clrlwi  r3,r4,31
---Type <return> to continue, or q <return> to quit---
   0x206fa460 <+608>:	stb     r4,199(r1)
   0x206fa464 <+612>:	cmplwi  r3,0
   0x206fa468 <+616>:	bne-    0x206fa734 <core::sync::atomic::atomic_compare_exchange+1332>
   0x206fa46c <+620>:	b       0x206fa720 <core::sync::atomic::atomic_compare_exchange+1312>
   0x206fa470 <+624>:	.long 0x7c2004ac
   0x206fa474 <+628>:	lwz     r3,68(r1)
   0x206fa478 <+632>:	lwz     r4,72(r1)
   0x206fa47c <+636>:	lwarx   r4,0,r4
   0x206fa480 <+640>:	stw     r4,28(r1)
   0x206fa484 <+644>:	cmpw    r3,r4
   0x206fa488 <+648>:	bne-    0x206fa4a0 <core::sync::atomic::atomic_compare_exchange+672>
   0x206fa48c <+652>:	lwz     r3,64(r1)
   0x206fa490 <+656>:	lwz     r4,72(r1)
   0x206fa494 <+660>:	stwcx.  r3,0,r4
   0x206fa498 <+664>:	bne+    0x206fa474 <core::sync::atomic::atomic_compare_exchange+628>
   0x206fa49c <+668>:	b       0x206fa4ac <core::sync::atomic::atomic_compare_exchange+684>
   0x206fa4a0 <+672>:	lwz     r3,28(r1)
   0x206fa4a4 <+676>:	lwz     r4,72(r1)
   0x206fa4a8 <+680>:	stwcx.  r3,0,r4
   0x206fa4ac <+684>:	lwz     r4,28(r1)
   0x206fa4b0 <+688>:	lwz     r3,68(r1)
   0x206fa4b4 <+692>:	xor     r3,r4,r3
   0x206fa4b8 <+696>:	cntlzw  r3,r3
   0x206fa4bc <+700>:	rlwinm  r3,r3,27,5,31
   0x206fa4c0 <+704>:	.long 0x7c2004ac
   0x206fa4c4 <+708>:	stw     r4,88(r1)
   0x206fa4c8 <+712>:	stb     r3,92(r1)
   0x206fa4cc <+716>:	b       0x206fa44c <core::sync::atomic::atomic_compare_exchange+588>
   0x206fa4d0 <+720>:	.long 0x7c2004ac
   0x206fa4d4 <+724>:	lwz     r3,68(r1)
   0x206fa4d8 <+728>:	lwz     r4,72(r1)
   0x206fa4dc <+732>:	lwarx   r4,0,r4
   0x206fa4e0 <+736>:	stw     r4,24(r1)
   0x206fa4e4 <+740>:	cmpw    r3,r4
   0x206fa4e8 <+744>:	bne-    0x206fa500 <core::sync::atomic::atomic_compare_exchange+768>
   0x206fa4ec <+748>:	lwz     r3,64(r1)
   0x206fa4f0 <+752>:	lwz     r4,72(r1)
   0x206fa4f4 <+756>:	stwcx.  r3,0,r4
   0x206fa4f8 <+760>:	bne+    0x206fa4d4 <core::sync::atomic::atomic_compare_exchange+724>
   0x206fa4fc <+764>:	b       0x206fa50c <core::sync::atomic::atomic_compare_exchange+780>
   0x206fa500 <+768>:	lwz     r3,24(r1)
   0x206fa504 <+772>:	lwz     r4,72(r1)
   0x206fa508 <+776>:	stwcx.  r3,0,r4
   0x206fa50c <+780>:	lwz     r4,24(r1)
   0x206fa510 <+784>:	lwz     r3,68(r1)
   0x206fa514 <+788>:	xor     r3,r4,r3
   0x206fa518 <+792>:	cntlzw  r3,r3
   0x206fa51c <+796>:	rlwinm  r3,r3,27,5,31
   0x206fa520 <+800>:	.long 0x7c2004ac
   0x206fa524 <+804>:	stw     r4,88(r1)
   0x206fa528 <+808>:	stb     r3,92(r1)
---Type <return> to continue, or q <return> to quit---
   0x206fa52c <+812>:	b       0x206fa44c <core::sync::atomic::atomic_compare_exchange+588>
   0x206fa530 <+816>:	lwz     r3,68(r1)
   0x206fa534 <+820>:	lwz     r4,72(r1)
   0x206fa538 <+824>:	lwarx   r4,0,r4
   0x206fa53c <+828>:	stw     r4,20(r1)
   0x206fa540 <+832>:	cmpw    r3,r4
   0x206fa544 <+836>:	bne-    0x206fa55c <core::sync::atomic::atomic_compare_exchange+860>
   0x206fa548 <+840>:	lwz     r3,64(r1)
   0x206fa54c <+844>:	lwz     r4,72(r1)
   0x206fa550 <+848>:	stwcx.  r3,0,r4
   0x206fa554 <+852>:	bne+    0x206fa530 <core::sync::atomic::atomic_compare_exchange+816>
   0x206fa558 <+856>:	b       0x206fa568 <core::sync::atomic::atomic_compare_exchange+872>
   0x206fa55c <+860>:	lwz     r3,20(r1)
   0x206fa560 <+864>:	lwz     r4,72(r1)
   0x206fa564 <+868>:	stwcx.  r3,0,r4
   0x206fa568 <+872>:	lwz     r4,20(r1)
   0x206fa56c <+876>:	lwz     r3,68(r1)
   0x206fa570 <+880>:	xor     r3,r4,r3
   0x206fa574 <+884>:	cntlzw  r3,r3
   0x206fa578 <+888>:	rlwinm  r3,r3,27,5,31
=> 0x206fa57c <+892>:	.long 0x7c2004ac
   0x206fa580 <+896>:	stw     r4,88(r1)
   0x206fa584 <+900>:	stb     r3,92(r1)
   0x206fa588 <+904>:	b       0x206fa44c <core::sync::atomic::atomic_compare_exchange+588>
   0x206fa58c <+908>:	lwz     r3,68(r1)
   0x206fa590 <+912>:	lwz     r4,72(r1)
   0x206fa594 <+916>:	lwarx   r4,0,r4
   0x206fa598 <+920>:	stw     r4,16(r1)
   0x206fa59c <+924>:	cmpw    r3,r4
   0x206fa5a0 <+928>:	bne-    0x206fa5b8 <core::sync::atomic::atomic_compare_exchange+952>
   0x206fa5a4 <+932>:	lwz     r3,64(r1)
   0x206fa5a8 <+936>:	lwz     r4,72(r1)
   0x206fa5ac <+940>:	stwcx.  r3,0,r4
   0x206fa5b0 <+944>:	bne+    0x206fa58c <core::sync::atomic::atomic_compare_exchange+908>
   0x206fa5b4 <+948>:	b       0x206fa5c4 <core::sync::atomic::atomic_compare_exchange+964>
   0x206fa5b8 <+952>:	lwz     r3,16(r1)
   0x206fa5bc <+956>:	lwz     r4,72(r1)
   0x206fa5c0 <+960>:	stwcx.  r3,0,r4
   0x206fa5c4 <+964>:	lwz     r4,16(r1)
   0x206fa5c8 <+968>:	lwz     r3,68(r1)
   0x206fa5cc <+972>:	xor     r3,r4,r3
   0x206fa5d0 <+976>:	cntlzw  r3,r3
   0x206fa5d4 <+980>:	rlwinm  r3,r3,27,5,31
   0x206fa5d8 <+984>:	.long 0x7c2004ac
   0x206fa5dc <+988>:	stw     r4,88(r1)
   0x206fa5e0 <+992>:	stb     r3,92(r1)
   0x206fa5e4 <+996>:	b       0x206fa44c <core::sync::atomic::atomic_compare_exchange+588>
   0x206fa5e8 <+1000>:	.long 0x7c2004ac
   0x206fa5ec <+1004>:	lwz     r3,68(r1)
   0x206fa5f0 <+1008>:	lwz     r4,72(r1)
   0x206fa5f4 <+1012>:	lwarx   r4,0,r4
---Type <return> to continue, or q <return> to quit---
   0x206fa5f8 <+1016>:	stw     r4,12(r1)
   0x206fa5fc <+1020>:	cmpw    r3,r4
   0x206fa600 <+1024>:	bne-    0x206fa618 <core::sync::atomic::atomic_compare_exchange+1048>
   0x206fa604 <+1028>:	lwz     r3,64(r1)
   0x206fa608 <+1032>:	lwz     r4,72(r1)
   0x206fa60c <+1036>:	stwcx.  r3,0,r4
   0x206fa610 <+1040>:	bne+    0x206fa5ec <core::sync::atomic::atomic_compare_exchange+1004>
   0x206fa614 <+1044>:	b       0x206fa624 <core::sync::atomic::atomic_compare_exchange+1060>
   0x206fa618 <+1048>:	lwz     r3,12(r1)
   0x206fa61c <+1052>:	lwz     r4,72(r1)
   0x206fa620 <+1056>:	stwcx.  r3,0,r4
   0x206fa624 <+1060>:	lwz     r4,12(r1)
   0x206fa628 <+1064>:	lwz     r3,68(r1)
   0x206fa62c <+1068>:	xor     r3,r4,r3
   0x206fa630 <+1072>:	cntlzw  r3,r3
   0x206fa634 <+1076>:	rlwinm  r3,r3,27,5,31
   0x206fa638 <+1080>:	stw     r4,88(r1)
   0x206fa63c <+1084>:	stb     r3,92(r1)
   0x206fa640 <+1088>:	b       0x206fa44c <core::sync::atomic::atomic_compare_exchange+588>
   0x206fa644 <+1092>:	lwz     r3,68(r1)
   0x206fa648 <+1096>:	lwz     r4,72(r1)
   0x206fa64c <+1100>:	lwarx   r4,0,r4
   0x206fa650 <+1104>:	stw     r4,8(r1)
   0x206fa654 <+1108>:	cmpw    r3,r4
   0x206fa658 <+1112>:	bne-    0x206fa670 <core::sync::atomic::atomic_compare_exchange+1136>
   0x206fa65c <+1116>:	lwz     r3,64(r1)
   0x206fa660 <+1120>:	lwz     r4,72(r1)
   0x206fa664 <+1124>:	stwcx.  r3,0,r4
   0x206fa668 <+1128>:	bne+    0x206fa644 <core::sync::atomic::atomic_compare_exchange+1092>
   0x206fa66c <+1132>:	b       0x206fa67c <core::sync::atomic::atomic_compare_exchange+1148>
   0x206fa670 <+1136>:	lwz     r3,8(r1)
   0x206fa674 <+1140>:	lwz     r4,72(r1)
   0x206fa678 <+1144>:	stwcx.  r3,0,r4
   0x206fa67c <+1148>:	lwz     r4,8(r1)
   0x206fa680 <+1152>:	lwz     r3,68(r1)
   0x206fa684 <+1156>:	xor     r3,r4,r3
   0x206fa688 <+1160>:	cntlzw  r3,r3
   0x206fa68c <+1164>:	rlwinm  r3,r3,27,5,31
   0x206fa690 <+1168>:	stw     r4,88(r1)
   0x206fa694 <+1172>:	stb     r3,92(r1)
   0x206fa698 <+1176>:	b       0x206fa44c <core::sync::atomic::atomic_compare_exchange+588>
   0x206fa69c <+1180>:	lwz     r4,-32628(r30)
   0x206fa6a0 <+1184>:	lwz     r6,-32704(r30)
   0x206fa6a4 <+1188>:	addi    r3,r1,152
   0x206fa6a8 <+1192>:	li      r5,1
   0x206fa6ac <+1196>:	li      r7,0
   0x206fa6b0 <+1200>:	bl      0x20710f08 <core::fmt::Arguments::new_v1>
   0x206fa6b4 <+1204>:	b       0x206fa6f0 <core::sync::atomic::atomic_compare_exchange+1264>
   0x206fa6b8 <+1208>:	lwz     r4,-32624(r30)
   0x206fa6bc <+1212>:	lwz     r6,-32704(r30)
   0x206fa6c0 <+1216>:	addi    r3,r1,128
---Type <return> to continue, or q <return> to quit---
   0x206fa6c4 <+1220>:	li      r5,1
   0x206fa6c8 <+1224>:	li      r7,0
   0x206fa6cc <+1228>:	bl      0x20710f08 <core::fmt::Arguments::new_v1>
   0x206fa6d0 <+1232>:	b       0x206fa710 <core::sync::atomic::atomic_compare_exchange+1296>
   0x206fa6d4 <+1236>:	lwz     r4,-32620(r30)
   0x206fa6d8 <+1240>:	lwz     r6,-32704(r30)
   0x206fa6dc <+1244>:	addi    r3,r1,104
   0x206fa6e0 <+1248>:	li      r5,1
   0x206fa6e4 <+1252>:	li      r7,0
   0x206fa6e8 <+1256>:	bl      0x20710f08 <core::fmt::Arguments::new_v1>
   0x206fa6ec <+1260>:	b       0x206fa700 <core::sync::atomic::atomic_compare_exchange+1280>
   0x206fa6f0 <+1264>:	lwz     r4,-32616(r30)
   0x206fa6f4 <+1268>:	addi    r3,r1,152
   0x206fa6f8 <+1272>:	bl      0x207e9064 <core::panicking::panic_fmt>
   0x206fa6fc <+1276>:	trap
   0x206fa700 <+1280>:	lwz     r4,-32612(r30)
   0x206fa704 <+1284>:	addi    r3,r1,104
   0x206fa708 <+1288>:	bl      0x207e9064 <core::panicking::panic_fmt>
   0x206fa70c <+1292>:	trap
   0x206fa710 <+1296>:	lwz     r4,-32608(r30)
   0x206fa714 <+1300>:	addi    r3,r1,128
   0x206fa718 <+1304>:	bl      0x207e9064 <core::panicking::panic_fmt>
   0x206fa71c <+1308>:	trap
   0x206fa720 <+1312>:	lwz     r3,32(r1)
   0x206fa724 <+1316>:	stw     r3,84(r1)
   0x206fa728 <+1320>:	li      r3,1
   0x206fa72c <+1324>:	stw     r3,80(r1)
   0x206fa730 <+1328>:	b       0x206fa748 <core::sync::atomic::atomic_compare_exchange+1352>
   0x206fa734 <+1332>:	lwz     r3,32(r1)
   0x206fa738 <+1336>:	stw     r3,84(r1)
   0x206fa73c <+1340>:	li      r3,0
   0x206fa740 <+1344>:	stw     r3,80(r1)
   0x206fa744 <+1348>:	b       0x206fa748 <core::sync::atomic::atomic_compare_exchange+1352>
   0x206fa748 <+1352>:	lwz     r3,80(r1)
   0x206fa74c <+1356>:	lwz     r4,84(r1)
   0x206fa750 <+1360>:	lwz     r0,212(r1)
   0x206fa754 <+1364>:	lwz     r30,200(r1)
   0x206fa758 <+1368>:	addi    r1,r1,208
   0x206fa75c <+1372>:	mtlr    r0
   0x206fa760 <+1376>:	blr
End of assembler dump.
