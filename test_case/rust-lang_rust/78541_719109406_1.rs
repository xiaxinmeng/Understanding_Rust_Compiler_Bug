
	push	rax
	cmp	rsi, rdx
	mov	rax, rdx
	cmovb	rax, rsi
	cmovb	rsi, rdx
	mov	rcx, rsi
	sub	rcx, rax
	jbe	.LBB7_6 ;; The manual panic
# %bb.1:
	cmp	rsi, 64
	jae	.LBB7_6 ;; The manual panic
# %bb.2:
	cmp	rax, 65
	jae	.LBB7_7 ;; "assertion failed: mid <= self.len()"
# %bb.3:
	mov	edx, 64
	sub	rdx, rax
	cmp	rdx, rcx
	jb	.LBB7_7 ;; "assertion failed: mid <= self.len()"
# %bb.4:
	test	rcx, rcx
	je	.LBB7_8 ;; to core::panicking::panic_bounds_check@GOTPCREL
# %bb.5:
	lea	rdx, [rdi + 4*rsi]
	lea	rax, [rdi + 4*rax]
	pop	rcx
	ret
