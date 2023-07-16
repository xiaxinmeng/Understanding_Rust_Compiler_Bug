asm
0000000000000000 <_ZN3bug17fake_thread_local17LocalKey$LT$T$GT$4with17hc98bdf741300d0b2E>:
   0:	50                   	push   %rax
   1:	ff 17                	call   *(%rdi)
   3:	48 89 04 24          	mov    %rax,(%rsp)
   7:	48 83 f8 00          	cmp    $0x0,%rax
   b:	75 1e                	jne    2b <_ZN3bug17fake_thread_local17LocalKey$LT$T$GT$4with17hc98bdf741300d0b2E+0x2b>
   d:	48 bf 00 00 00 00 00 	movabs $0x0,%rdi
  14:	00 00 00 
  17:	31 c0                	xor    %eax,%eax
  19:	89 c6                	mov    %eax,%esi
  1b:	48 ba 00 00 00 00 00 	movabs $0x0,%rdx
  22:	00 00 00 
  25:	ff 15 00 00 00 00    	call   *0x0(%rip)        # 2b <_ZN3bug17fake_thread_local17LocalKey$LT$T$GT$4with17hc98bdf741300d0b2E+0x2b>
  2b:	48 8b 3c 24          	mov    (%rsp),%rdi
