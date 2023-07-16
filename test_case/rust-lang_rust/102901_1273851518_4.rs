
0000000000000000 <enum_fair::disco>:
   0:	40 80 c7 fe          	add    dil,0xfe
   4:	40 0f b6 cf          	movzx  ecx,dil
   8:	48 8d 51 01          	lea    rdx,[rcx+0x1]
   c:	31 c0                	xor    eax,eax
   e:	80 f9 03             	cmp    cl,0x3
  11:	48 0f 42 c2          	cmovb  rax,rdx
  15:	c3                   	ret    
