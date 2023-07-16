asm
Dump of assembler code for function _ZNK4llvm5APInt4rotlEj:
   0x00007fffeff32140 <+0>:	push   %r14
   0x00007fffeff32142 <+2>:	mov    %edx,%eax
   0x00007fffeff32144 <+4>:	xor    %edx,%edx
   0x00007fffeff32146 <+6>:	push   %r13
   0x00007fffeff32148 <+8>:	push   %r12
   0x00007fffeff3214a <+10>:	push   %rbp
   0x00007fffeff3214b <+11>:	mov    %rsi,%rbp
   0x00007fffeff3214e <+14>:	push   %rbx
   0x00007fffeff3214f <+15>:	mov    %rdi,%rbx
   0x00007fffeff32152 <+18>:	sub    $0x20,%rsp
   0x00007fffeff32156 <+22>:	mov    (%rsi),%r8d
=> 0x00007fffeff32159 <+25>:	div    %r8d
   0x00007fffeff3215c <+28>:	test   %edx,%edx
   0x00007fffeff3215e <+30>:	mov    %edx,%r12d
   0x00007fffeff32161 <+33>:	jne    0x7fffeff32190 <_ZNK4llvm5APInt4rotlEj+80>
   0x00007fffeff32163 <+35>:	cmp    $0x40,%r8d
   0x00007fffeff32167 <+39>:	mov    %r8d,(%rdi)
   0x00007fffeff3216a <+42>:	movq   $0x0,0x8(%rdi)
   0x00007fffeff32172 <+50>:	ja     0x7fffeff32238 <_ZNK4llvm5APInt4rotlEj+248>
   0x00007fffeff32178 <+56>:	mov    0x8(%rsi),%rax
   0x00007fffeff3217c <+60>:	mov    %rax,0x8(%rdi)
   0x00007fffeff32180 <+64>:	add    $0x20,%rsp
   0x00007fffeff32184 <+68>:	mov    %rbx,%rax
   0x00007fffeff32187 <+71>:	pop    %rbx
   0x00007fffeff32188 <+72>:	pop    %rbp
   0x00007fffeff32189 <+73>:	pop    %r12
   0x00007fffeff3218b <+75>:	pop    %r13
   0x00007fffeff3218d <+77>:	pop    %r14
   0x00007fffeff3218f <+79>:	retq   
   0x00007fffeff32190 <+80>:	lea    0x10(%rsp),%r13
   0x00007fffeff32195 <+85>:	mov    %r8d,%edx
   0x00007fffeff32198 <+88>:	sub    %r12d,%edx
   0x00007fffeff3219b <+91>:	mov    %r13,%rdi
   0x00007fffeff3219e <+94>:	callq  0x7fffee7b3910 <_ZNK4llvm5APInt4lshrEj@plt>
   0x00007fffeff321a3 <+99>:	mov    0x0(%rbp),%eax
   0x00007fffeff321a6 <+102>:	cmp    $0x40,%eax
   0x00007fffeff321a9 <+105>:	ja     0x7fffeff322b0 <_ZNK4llvm5APInt4rotlEj+368>
   0x00007fffeff321af <+111>:	cmp    %eax,%r12d
   0x00007fffeff321b2 <+114>:	jae    0x7fffeff32250 <_ZNK4llvm5APInt4rotlEj+272>
   0x00007fffeff321b8 <+120>:	mov    0x8(%rbp),%rdx
   0x00007fffeff321bc <+124>:	mov    %r12d,%ecx
   0x00007fffeff321bf <+127>:	mov    %rsp,%rdi
   0x00007fffeff321c2 <+130>:	mov    %eax,(%rsp)
   0x00007fffeff321c5 <+133>:	mov    %rsp,%r14
   0x00007fffeff321c8 <+136>:	shl    %cl,%rdx
   0x00007fffeff321cb <+139>:	mov    %rdx,0x8(%rsp)
   0x00007fffeff321d0 <+144>:	callq  0x7fffee94cab0 <_ZN4llvm5APInt15clearUnusedBitsEv>
   0x00007fffeff321d5 <+149>:	mov    (%rsp),%eax
   0x00007fffeff321d8 <+152>:	cmp    $0x40,%eax
   0x00007fffeff321db <+155>:	jbe    0x7fffeff32273 <_ZNK4llvm5APInt4rotlEj+307>
   0x00007fffeff321e1 <+161>:	mov    %r13,%rdx
   0x00007fffeff321e4 <+164>:	mov    %r14,%rsi
   0x00007fffeff321e7 <+167>:	mov    %rbx,%rdi
   0x00007fffeff321ea <+170>:	callq  0x7fffee7c1290 <_ZNK4llvm5APInt10OrSlowCaseERKS0_@plt>
   0x00007fffeff321ef <+175>:	cmpl   $0x40,(%rsp)
   0x00007fffeff321f3 <+179>:	jbe    0x7fffeff32204 <_ZNK4llvm5APInt4rotlEj+196>
   0x00007fffeff321f5 <+181>:	mov    0x8(%rsp),%rdi
   0x00007fffeff321fa <+186>:	test   %rdi,%rdi
   0x00007fffeff321fd <+189>:	je     0x7fffeff32204 <_ZNK4llvm5APInt4rotlEj+196>
   0x00007fffeff321ff <+191>:	callq  0x7fffee7e99f0 <_ZdaPv@plt>
   0x00007fffeff32204 <+196>:	cmpl   $0x40,0x10(%rsp)
   0x00007fffeff32209 <+201>:	jbe    0x7fffeff32180 <_ZNK4llvm5APInt4rotlEj+64>
   0x00007fffeff3220f <+207>:	mov    0x18(%rsp),%rdi
   0x00007fffeff32214 <+212>:	test   %rdi,%rdi
   0x00007fffeff32217 <+215>:	je     0x7fffeff32180 <_ZNK4llvm5APInt4rotlEj+64>
   0x00007fffeff3221d <+221>:	callq  0x7fffee7e99f0 <_ZdaPv@plt>
   0x00007fffeff32222 <+226>:	add    $0x20,%rsp
   0x00007fffeff32226 <+230>:	mov    %rbx,%rax
   0x00007fffeff32229 <+233>:	pop    %rbx
   0x00007fffeff3222a <+234>:	pop    %rbp
   0x00007fffeff3222b <+235>:	pop    %r12
   0x00007fffeff3222d <+237>:	pop    %r13
   0x00007fffeff3222f <+239>:	pop    %r14
   0x00007fffeff32231 <+241>:	retq   
   0x00007fffeff32232 <+242>:	nopw   0x0(%rax,%rax,1)
   0x00007fffeff32238 <+248>:	callq  0x7fffee7d1d20 <_ZN4llvm5APInt12initSlowCaseERKS0_@plt>
   0x00007fffeff3223d <+253>:	add    $0x20,%rsp
   0x00007fffeff32241 <+257>:	mov    %rbx,%rax
   0x00007fffeff32244 <+260>:	pop    %rbx
   0x00007fffeff32245 <+261>:	pop    %rbp
   0x00007fffeff32246 <+262>:	pop    %r12
   0x00007fffeff32248 <+264>:	pop    %r13
   0x00007fffeff3224a <+266>:	pop    %r14
   0x00007fffeff3224c <+268>:	retq   
   0x00007fffeff3224d <+269>:	nopl   (%rax)
   0x00007fffeff32250 <+272>:	mov    %rsp,%rdi
   0x00007fffeff32253 <+275>:	mov    %eax,(%rsp)
   0x00007fffeff32256 <+278>:	movq   $0x0,0x8(%rsp)
   0x00007fffeff3225f <+287>:	callq  0x7fffee94cab0 <_ZN4llvm5APInt15clearUnusedBitsEv>
   0x00007fffeff32264 <+292>:	mov    (%rsp),%eax
   0x00007fffeff32267 <+295>:	mov    %rsp,%r14
   0x00007fffeff3226a <+298>:	cmp    $0x40,%eax
   0x00007fffeff3226d <+301>:	ja     0x7fffeff321e1 <_ZNK4llvm5APInt4rotlEj+161>
   0x00007fffeff32273 <+307>:	mov    0x18(%rsp),%rdx
   0x00007fffeff32278 <+312>:	or     0x8(%rsp),%rdx
   0x00007fffeff3227d <+317>:	mov    %eax,(%rbx)
   0x00007fffeff3227f <+319>:	and    $0x3f,%eax
   0x00007fffeff32282 <+322>:	mov    %rdx,0x8(%rbx)
   0x00007fffeff32286 <+326>:	je     0x7fffeff32204 <_ZNK4llvm5APInt4rotlEj+196>
   0x00007fffeff3228c <+332>:	mov    $0x40,%ecx
   0x00007fffeff32291 <+337>:	sub    %eax,%ecx
   0x00007fffeff32293 <+339>:	mov    $0xffffffffffffffff,%rax
   0x00007fffeff3229a <+346>:	shr    %cl,%rax
   0x00007fffeff3229d <+349>:	and    %rdx,%rax
   0x00007fffeff322a0 <+352>:	mov    %rax,0x8(%rbx)
   0x00007fffeff322a4 <+356>:	jmpq   0x7fffeff32204 <_ZNK4llvm5APInt4rotlEj+196>
   0x00007fffeff322a9 <+361>:	nopl   0x0(%rax)
   0x00007fffeff322b0 <+368>:	mov    %r12d,%edx
   0x00007fffeff322b3 <+371>:	mov    %rbp,%rsi
   0x00007fffeff322b6 <+374>:	mov    %rsp,%rdi
   0x00007fffeff322b9 <+377>:	mov    %rsp,%r14
   0x00007fffeff322bc <+380>:	callq  0x7fffee7b3b90 <_ZNK4llvm5APInt11shlSlowCaseEj@plt>
   0x00007fffeff322c1 <+385>:	jmpq   0x7fffeff321d5 <_ZNK4llvm5APInt4rotlEj+149>
End of assembler dump.
