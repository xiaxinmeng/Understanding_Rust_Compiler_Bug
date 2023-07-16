
0000000000000000 <enum_fair::bar>:
   0:	40 80 c7 fb          	add    dil,0xfb
   4:	40 80 ff fd          	cmp    dil,0xfd
   8:	0f 92 c0             	setb   al
   b:	c3                   	ret    

0000000000000000 <abx_bad::x>:
   0:	8d 8f 00 00 ef ff    	lea    ecx,[rdi-0x110000]
   6:	83 f9 01             	cmp    ecx,0x1
   9:	b8 78 00 00 00       	mov    eax,0x78
   e:	0f 44 c7             	cmove  eax,edi
  11:	83 f9 03             	cmp    ecx,0x3
  14:	0f 43 c7             	cmovae eax,edi
  17:	c3                   	ret    
