
   0x00007ffff49a2303 <+3411>:	je     0x7ffff49a237b <_ZN5rustc3mir14BasicBlockData17expand_statements17hf1d4ebe84ecd6e03E+3531>
   0x00007ffff49a2305 <+3413>:	mov    0x100(%rsp),%rax
   0x00007ffff49a230d <+3421>:	mov    %rax,0xa0(%rsp)
   0x00007ffff49a2315 <+3429>:	movaps 0xf0(%rsp),%xmm0
   0x00007ffff49a231d <+3437>:	movaps %xmm0,0x90(%rsp)
   0x00007ffff49a2325 <+3445>:	movaps 0xb0(%rsp),%xmm0
   0x00007ffff49a232d <+3453>:	movaps 0xc0(%rsp),%xmm1
   0x00007ffff49a2335 <+3461>:	movdqa 0xd0(%rsp),%xmm2
   0x00007ffff49a233e <+3470>:	movaps 0xe0(%rsp),%xmm3
   0x00007ffff49a2346 <+3478>:	movaps %xmm3,0x80(%rsp)
   0x00007ffff49a234e <+3486>:	movdqa %xmm2,0x70(%rsp)
   0x00007ffff49a2354 <+3492>:	movaps %xmm1,0x60(%rsp)
   0x00007ffff49a2359 <+3497>:	movaps %xmm0,0x50(%rsp)
   0x00007ffff49a235e <+3502>:	mov    %r12,%rdi
   0x00007ffff49a2361 <+3505>:	callq  0x7ffff46c8880 <_ZN4core3ptr13drop_in_place17h3188964708fd179cE.llvm.12071625485570946806>
   0x00007ffff49a2366 <+3510>:	jmpq   0x7ffff49a2290 <_ZN5rustc3mir14BasicBlockData17expand_statements17hf1d4ebe84ecd6e03E+3296>
   0x00007ffff49a236b <+3515>:	nopl   0x0(%rax,%rax,1)
   0x00007ffff49a2370 <+3520>:	movl   $0x9,0xb0(%rsp)
   0x00007ffff49a237b <+3531>:	mov    0x368(%rsp),%rax
   0x00007ffff49a2383 <+3539>:	test   %rax,%rax
   0x00007ffff49a2386 <+3542>:	je     0x7ffff49a23c4 <_ZN5rustc3mir14BasicBlockData17expand_statements17hf1d4ebe84ecd6e03E+3604>
   0x00007ffff49a2388 <+3544>:	mov    0x380(%rsp),%r15
   0x00007ffff49a2390 <+3552>:	mov    (%r15),%rcx
   0x00007ffff49a2393 <+3555>:	mov    0x10(%r15),%rbx
   0x00007ffff49a2397 <+3559>:	imul   $0x58,0x360(%rsp),%rsi
   0x00007ffff49a23a0 <+3568>:	add    %rcx,%rsi
   0x00007ffff49a23a3 <+3571>:	imul   $0x58,%rbx,%rdi
   0x00007ffff49a23a7 <+3575>:	add    %rcx,%rdi
   0x00007ffff49a23aa <+3578>:	imul   $0x58,%rax,%rdx
   0x00007ffff49a23ae <+3582>:	callq  0x7ffff46be350 <memmove@plt>
=> 0x00007ffff49a23b3 <+3587>:	add    0x368(%rsp),%rbx
   0x00007ffff49a23bb <+3595>:	mov    %rbx,0x10(%r15)
   0x00007ffff49a23bf <+3599>:	mov    0x28(%rsp),%r15
   0x00007ffff49a23c4 <+3604>:	mov    %r14,%rdi
   0x00007ffff49a23c7 <+3607>:	callq  0x7ffff46e3080 <_ZN71_$LT$alloc..vec..IntoIter$LT$T$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h62e13b79d8b9a38aE>
   0x00007ffff49a23cc <+3612>:	mov    0x3b0(%rsp),%rdi
   0x00007ffff49a23d4 <+3620>:	mov    $0x38,%esi
   0x00007ffff49a23d9 <+3625>:	mov    $0x8,%edx
   0x00007ffff49a23de <+3630>:	callq  0x7ffff46bbd40 <__rust_dealloc@plt>
   0x00007ffff49a23e3 <+3635>:	lea    0x3b8(%rsp),%rdi
   0x00007ffff49a23eb <+3643>:	callq  0x7ffff4993fd0 <_ZN4core3ptr13drop_in_place17hdc52da31c044fba0E.llvm.13745968970215452383>
   0x00007ffff49a23f0 <+3648>:	cmpl   $0x9,0x3f8(%rsp)
   0x00007ffff49a23f8 <+3656>:	je     0x7ffff49a2407 <_ZN5rustc3mir14BasicBlockData17expand_statements17hf1d4ebe84ecd6e03E+3671>
   0x00007ffff49a23fa <+3658>:	lea    0x3f8(%rsp),%rdi
   0x00007ffff49a2402 <+3666>:	callq  0x7ffff49911e0 <_ZN4core3ptr13drop_in_place17h3188964708fd179cE>
   0x00007ffff49a2407 <+3671>:	mov    0x48(%rsp),%rsi
   0x00007ffff49a240c <+3676>:	cmp    0x40(%rsp),%rsi
   0x00007ffff49a2411 <+3681>:	lea    0x120(%rsp),%r14
   0x00007ffff49a2419 <+3689>:	jne    0x7ffff49a1ff3 <_ZN5rustc3mir14BasicBlockData17expand_statements17hf1d4ebe84ecd6e03E+2627>
   0x00007ffff49a241f <+3695>:	lea    0x30(%rsp),%rdi
   0x00007ffff49a2424 <+3700>:	callq  0x7ffff46e2b30 <_ZN71_$LT$alloc..vec..IntoIter$LT$T$GT$$u20$as$u20$core..ops..drop..Drop$GT$4drop17h3943334a18285159E>
   0x00007ffff49a2429 <+3705>:	lea    -0x28(%rbp),%rsp
   0x00007ffff49a242d <+3709>:	pop    %rbx
   0x00007ffff49a242e <+3710>:	pop    %r12
   0x00007ffff49a2430 <+3712>:	pop    %r13
   0x00007ffff49a2432 <+3714>:	pop    %r14
   0x00007ffff49a2434 <+3716>:	pop    %r15
   0x00007ffff49a2436 <+3718>:	pop    %rbp
   0x00007ffff49a2437 <+3719>:	retq   
   0x00007ffff49a2438 <+3720>:	add    $0xffffffffffffffff,%rsi
   0x00007ffff49a243c <+3724>:	lea    0x31231d(%rip),%rdi        # 0x7ffff4cb4760
   0x00007ffff49a2443 <+3731>:	callq  0x7ffff46bc360 <_ZN4core9panicking18panic_bounds_check17hcc582842c20cc97bE@plt>
   0x00007ffff49a2448 <+3736>:	jmp    0x7ffff49a245d <_ZN5rustc3mir14BasicBlockData17expand_statements17hf1d4ebe84ecd6e03E+3757>
   0x00007ffff49a244a <+3738>:	add    $0xffffffffffffffff,%r15
   0x00007ffff49a244e <+3742>:	lea    0x312323(%rip),%rdi        # 0x7ffff4cb4778
   0x00007ffff49a2455 <+3749>:	mov    %r15,%rsi
   0x00007ffff49a2458 <+3752>:	callq  0x7ffff46bc360 <_ZN4core9panicking18panic_bounds_check17hcc582842c20cc97bE@plt>
   0x00007ffff49a245d <+3757>:	ud2    
   0x00007ffff49a245f <+3759>:	lea    0x312c52(%rip),%rax        # 0x7ffff4cb50b8
   0x00007ffff49a2466 <+3766>:	mov    %rax,0x120(%rsp)
   0x00007ffff49a246e <+3774>:	movq   $0x1,0x128(%rsp)
   0x00007ffff49a247a <+3786>:	movq   $0x0,0x130(%rsp)
   0x00007ffff49a2486 <+3798>:	lea    0x324ab(%rip),%rax        # 0x7ffff49d4938 <byte_str.f.llvm.13745968970215452383>
