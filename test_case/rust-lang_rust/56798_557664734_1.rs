
0000000000000000 <cas128>:
   0:	55                   	push   %rbp
   1:	48 89 e5             	mov    %rsp,%rbp
   4:	53                   	push   %rbx
   5:	b8 2a 00 00 00       	mov    $0x2a,%eax
   a:	31 c9                	xor    %ecx,%ecx
   c:	89 ca                	mov    %ecx,%edx
   e:	b9 2b 00 00 00       	mov    $0x2b,%ecx
  13:	89 cb                	mov    %ecx,%ebx
  15:	48 89 55 f0          	mov    %rdx,-0x10(%rbp)
  19:	48 8b 4d f0          	mov    -0x10(%rbp),%rcx
  1d:	f0 48 0f c7 0d 00 00 	lock cmpxchg16b 0x0(%rip)        # 26 <cas128+0x26>
  24:	00 00 
  26:	48 89 45 e8          	mov    %rax,-0x18(%rbp)
  2a:	48 89 55 e0          	mov    %rdx,-0x20(%rbp)
  2e:	5b                   	pop    %rbx
  2f:	5d                   	pop    %rbp
  30:	c3                   	retq   
