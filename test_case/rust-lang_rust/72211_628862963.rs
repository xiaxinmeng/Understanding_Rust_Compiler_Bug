asm
00000000000502d0 <_ZN4test3foo17h1991a29ca1711cdfE>:
   502d0:	41 56                	push   r14
   502d2:	53                   	push   rbx
   502d3:	50                   	push   rax
   502d4:	49 89 f6             	mov    r14,rsi
   502d7:	48 89 fb             	mov    rbx,rdi
   502da:	ba 00 04 00 00       	mov    edx,0x400
   502df:	31 f6                	xor    esi,esi
   502e1:	ff 15 d1 d1 0d 00    	call   QWORD PTR [rip+0xdd1d1]        # 12d4b8 <memset@GLIBC_2.2.5>
   502e7:	48 89 df             	mov    rdi,rbx
   502ea:	41 ff d6             	call   r14
   502ed:	48 89 d8             	mov    rax,rbx
   502f0:	48 83 c4 08          	add    rsp,0x8
   502f4:	5b                   	pop    rbx
   502f5:	41 5e                	pop    r14
   502f7:	c3                   	ret    
   502f8:	0f 1f 84 00 00 00 00 	nop    DWORD PTR [rax+rax*1+0x0]
   502ff:	00 
