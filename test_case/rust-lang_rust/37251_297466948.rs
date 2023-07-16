asm
.LBB0_3:
	subq	%rdi, %rsi
	je	.LBB0_4      ; tail.is_empty()
	leaq	(%rax,%rdi,8), %rbx   ; the address being accessed
; rax points to s[0], rdi is head.len() -> rbx points to tail[0]
	cmpq	$2222222, (%rbx)      ; *** the memory access ***
	movl	$1, %ecx
	cmovaq	%r10, %rcx
	movl	$0, %edx
	cmovneq	%rcx, %rdx
	cmpq	$-1, %rdx
	je	.LBB0_8      ; Less, continue on tail
	testq	%rdx, %rdx
	je	.LBB0_11     ; Equal, exit loop
	movq	%rdi, %rsi ; s = head (implemented as s.len = head.len)
	jmp	.LBB0_9      ; Greater, continue on head
	.p2align	4, 0x90
.LBB0_8:
	leaq	1(%rdi,%r11), %r11
# tail[1..]
	addq	$8, %rbx ; add (the size of) one element to the start pointer
	decq	%rsi     ; subtract 1 to the length
	movq	%rbx, %rax ; and move it to s instead of tail
.LBB0_9:
; rax is the start pointer of s, rsi its length
	prefetcht0	(%rax)        ; *** the prefetch ***
	movq	%rsi, %rdi
	shrq	%rdi
	cmpq	%rdi, %rsi
	jae	.LBB0_3      ; LLVM was unable to remove the bound checks
	jmp	.LBB0_10     ; panic on illegal access
	.p2align	4, 0x90
