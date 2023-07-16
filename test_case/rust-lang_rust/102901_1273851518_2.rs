
0000000000000000 <enum_fair::bar>:
   0:	40 80 ff 02          	cmp    dil,0x2
   4:	0f 92 c0             	setb   al
   7:	c3                   	ret    

0000000000000000 <abx_bad::x>:
   0:	81 ff 00 00 11 00    	cmp    edi,0x110000
   6:	b8 78 00 00 00       	mov    eax,0x78
   b:	0f 42 c7             	cmovb  eax,edi
   e:	c3                   	ret    
