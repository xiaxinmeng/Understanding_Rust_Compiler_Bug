asm
0000000000005ae7 <add>:
    5ae7:	31 c9                	xor    ecx,ecx
    5ae9:	31 c0                	xor    eax,eax
    5aeb:	48 39 ce             	cmp    rsi,rcx
    5aee:	74 08                	je     5af8 <add+0x11>
    5af0:	03 04 8f             	add    eax,DWORD PTR [rdi+rcx*4]
    5af3:	48 ff c1             	inc    rcx
    5af6:	eb f3                	jmp    5aeb <add+0x4>
    5af8:	c3                   	ret    
