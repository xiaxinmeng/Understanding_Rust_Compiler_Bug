
0000000000000000 <enum_fair::disco>:
   0:	8d 47 ff             	lea    eax,[rdi-0x1]
   3:	31 c9                	xor    ecx,ecx
   5:	40 80 ff 02          	cmp    dil,0x2
   9:	0f b6 c0             	movzx  eax,al
   c:	0f 42 c1             	cmovb  eax,ecx
   f:	0f b6 c0             	movzx  eax,al
  12:	c3            
