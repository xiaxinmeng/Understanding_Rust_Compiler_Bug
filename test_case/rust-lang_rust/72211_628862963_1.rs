asm
00000000000492b0 <_ZN4test3foo17h1991a29ca1711cdfE>:
   492b0:	41 57                	push   r15
   492b2:	41 56                	push   r14
   492b4:	53                   	push   rbx
   492b5:	48 81 ec 00 04 00 00 	sub    rsp,0x400
   492bc:	49 89 f6             	mov    r14,rsi
   492bf:	48 89 fb             	mov    rbx,rdi
   492c2:	49 89 e7             	mov    r15,rsp
   492c5:	ba 00 04 00 00       	mov    edx,0x400
   492ca:	4c 89 ff             	mov    rdi,r15
   492cd:	31 f6                	xor    esi,esi
   492cf:	ff 15 43 93 0b 00    	call   QWORD PTR [rip+0xb9343]        # 102618 <memset@GLIBC_2.2.5>
   492d5:	4c 89 ff             	mov    rdi,r15
   492d8:	41 ff d6             	call   r14
   492db:	ba 00 04 00 00       	mov    edx,0x400
   492e0:	48 89 df             	mov    rdi,rbx
   492e3:	4c 89 fe             	mov    rsi,r15
   492e6:	ff 15 c4 95 0b 00    	call   QWORD PTR [rip+0xb95c4]        # 1028b0 <memcpy@GLIBC_2.14>
   492ec:	48 89 d8             	mov    rax,rbx
   492ef:	48 81 c4 00 04 00 00 	add    rsp,0x400
   492f6:	5b                   	pop    rbx
   492f7:	41 5e                	pop    r14
   492f9:	41 5f                	pop    r15
   492fb:	c3                   	ret    
   492fc:	0f 1f 40 00          	nop    DWORD PTR [rax+0x0]
